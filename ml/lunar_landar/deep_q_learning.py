from keras.layers import InputLayer, Dense, Activation, Conv1D
from keras.models import Sequential, load_model
from keras.optimizers import Adam
import numpy as np


class ReplayBuffer:
    def __init__(self, max_size, input_shape, n_actions):
        self.memory_size = max_size
        self.memory_counter = 0
        self.n_actions = n_actions
        self.state_memory = np.zeros((self.memory_size, input_shape))
        self.new_state_memory = np.zeros((self.memory_size, input_shape))
        self.action_memory = np.zeros((self.memory_size, n_actions), dtype=np.int8)
        self.reward_memory = np.zeros(self.memory_size)
        self.terminal_memory = np.zeros(self.memory_size, dtype=np.float32)

    def store_transition(self, state, action, reward, new_state, done):
        index = self.memory_counter % self.memory_size
        self.state_memory[index] = state
        self.new_state_memory[index] = new_state
        self.reward_memory[index] = reward
        self.terminal_memory[index] = 1 - int(done)
        actions = np.zeros((self.action_memory.shape[1]))
        actions[action] = 1
        self.action_memory[index] = actions
        self.memory_counter += 1

    def sample_buffer(self, batch_size):
        max_mem = min(self.memory_counter, self.memory_size)
        batch = np.random.choice(max_mem, batch_size)

        return (
            self.state_memory[batch],
            self.action_memory[batch],
            self.reward_memory[batch],
            self.new_state_memory[batch],
            self.terminal_memory[batch]
        )


def build_deep_q_network(learning_rate, n_actions, input_dims, dense_layers, conv_layers=None):
    model = Sequential()
    model.add(Dense(dense_layers[0], input_shape=(input_dims,)))
    model.add(Activation('relu'))

    for layer in dense_layers[1:]:
        model.add(Dense(layer))
        model.add(Activation('relu'))
    model.add(Dense(n_actions))

    # model = Sequential([
    #     Dense(256, input_shape=(input_dims, )),
    #     Activation('relu'),
    #     Dense(256),
    #     Activation('relu'),
    #     Dense(n_actions)
    # ])

    model.compile(optimizer=Adam(lr=learning_rate), loss='mse')
    return model


class DeepQAgent:
    def __init__(
            self,
            input_dims: int,  # number of input dimensions
            n_actions: int,  # number of actions
            alpha: float = 0.0005,  # learning rate
            gamma: float = 0.99,  # discount factor
            epsilon: float = 0.0,  # epsilon greedy factor
            n_batches: int = 64,  # number of batches
            epsilon_dec: float = 0.996,  # decrement factor epsilon greedy
            epsilon_min: float = 0.01,  # minimum value of epsilon greedy
            memory_size: int = 1000000,  # size of memory
            dense_layers: tuple = (36, 27),  # dims of hidden layers
            conv_layers: tuple = None,  # dims of hidden layers
            initial_model: str = None,  # file path to initial model
            replace_target: int = 100  # num of iter to replace the target
    ):
        self.alpha = alpha
        self.gamma = gamma
        self.epsilon = epsilon
        self.action_space = np.arange(n_actions, dtype=np.int8)
        self.n_batches = n_batches
        self.input_dims = input_dims
        self.epsilon_min = epsilon_min
        self.epsilon_dec = epsilon_dec
        self.memory_size = memory_size
        self.replace_target = replace_target

        self.memory = ReplayBuffer(memory_size, input_dims, n_actions)

        if initial_model is None:
            self.q_eval = build_deep_q_network(alpha, n_actions, input_dims, dense_layers, conv_layers)
            self.q_target = build_deep_q_network(alpha, n_actions, input_dims, dense_layers, conv_layers)
        else:
            self.q_eval = load_model(initial_model)
            self.q_target = load_model(initial_model)
            assert self.q_eval.input_shape[1] == input_dims
            assert self.q_eval.output_shape[1] == n_actions

    def remember(self, state, action, reward, new_sate, done):
        self.memory.store_transition(state, action, reward, new_sate, done)

    def choose_action(self, state):
        state = state[np.newaxis, :]
        if np.random.random() < self.epsilon:
            action = np.random.choice(self.action_space)
        else:
            actions = self.q_eval.predict(state)
            action = np.argmax(actions)

        return action

    def learn(self):
        if self.memory.memory_counter < self.n_batches:
            return

        state, action, reward, new_sate, done = self.memory.sample_buffer(self.n_batches)
        action_indices = np.dot(action, self.action_space)

        q_next = self.q_target.predict(new_sate)
        q_eval = self.q_eval.predict(new_sate)

        max_actions = np.argmax(q_eval, axis=1)
        batch_index = np.arange(self.n_batches, dtype=np.int32)

        q_target = self.q_eval.predict(state)
        q_target[batch_index, action_indices] = reward + self.gamma * q_next[batch_index, max_actions] * done

        self.q_eval.fit(state, q_target, verbose=0)

        if self.epsilon > self.epsilon_min:
            self.epsilon = self.epsilon * self.epsilon_dec
        else:
            self.epsilon = self.epsilon_min

        if self.memory.memory_counter % self.replace_target == 0:
            self.update_network_parameters()

    def update_network_parameters(self):
        self.q_target.set_weights(self.q_eval.get_weights())

    def save_model(self, file_path):
        self.q_eval.save(file_path)

    def load_model(self, file_path):
        self.q_eval = load_model(file_path)

        if self.epsilon <= self.epsilon_min:
            self.update_network_parameters()


class RandomAgent:
    def __init__(self, n_actions):
        self.action_space = np.arange(n_actions)

    def choose_action(self, state=None):
        return np.random.choice(self.action_space)
