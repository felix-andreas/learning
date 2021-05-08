import time
import gym
env = gym.make('Taxi-v2')
env.reset()

start = time.process_time()
done = True
while done:
    env.render()
    observation, reward, done, info = env.step(env.action_space.sample()) # take a random action
    print(f"\rreward:{reward} time: {time.process_time() - start}", end="")

