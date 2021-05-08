from deep_q_learning import DeepQAgent
import numpy as np
import gym

env = gym.make("LunarLander-v2")
file_path = "lunar_lander.h5"
n_games = 500
ai_agent = DeepQAgent(
    gamma=0.99,
    epsilon=1.0,
    epsilon_dec=0.996,
    alpha=0.0005,
    input_dims=8,
    n_actions=4,
    n_batches=64,
    dense_layers=(256, 256),
    replace_target=50,
    # initial_model=file_path,
)

n_saves = 10
scores = np.full(n_saves, -125)
scores_average = []
epsilons = []

for i in range(n_games):
    done = False
    score = 0
    state = env.reset()
    while not done:
        action = ai_agent.choose_action(state)
        new_state, reward, done, info = env.step(action)
        # env.render()
        score += reward
        ai_agent.remember(state, action, reward, new_state, done)
        ai_agent.learn()
        state = new_state

    scores[i % n_saves] = score
    scores_average.append(np.mean(scores))
    epsilons.append(ai_agent.epsilon)

    if i % n_saves == 0:
        print(
            f"game: {i}, avg_score: {np.mean(scores):.2f}, eps: {ai_agent.epsilon:.2f}"
        )
        ai_agent.save_model(file_path)

from plot import plot_scores
import matplotlib.pyplot as plt

fig = plot_scores(scores_average, epsilons)
plt.show()
