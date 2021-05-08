from keras.models import load_model
import numpy as np
import gym

model = load_model('lunar_lander_250.h5')
env = gym.make('LunarLander-v2')

while True:
    done = False
    score = 0

    state = env.reset()
    env.render()
    while not done:
        state = state[np.newaxis, :]
        actions = model.predict(state)
        action = np.argmax(actions)
        state, reward, done, info = env.step(action)
        score += reward
        env.render()
    print(score)
