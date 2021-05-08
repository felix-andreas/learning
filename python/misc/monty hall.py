import numpy as np

N = 50000
N_doors = 3
idx = np.arange(N)
doors = np.zeros((N, N_doors), dtype=np.bool)
choice1 = np.zeros((N, N_doors), dtype=np.bool)
rand1 = np.random.randint(N_doors, size=N)
rand2 = np.random.randint(N_doors, size=N)
doors[idx,rand1] = True
choice1[idx,rand2] = True
choice2= np.logical_not(doors +choice1)
for i in range(N):
    if sum(choice2[i,:] > 1):
        jvec = np.where(choice2[i,:] == True)
        randint = np.random.randint(1, size=1)
        j = jvec[randint]
        choice2[i,j] = False



choice2[idx, :] = np.logical_not(choice1[idx, :])
print("doors\n", doors)
print("choice 1\n", choice1)
print("doors = choice 1\n", doors +choice1)
print("choice 2\n", choice2)
print("probability choice 1\n", np.sum(doors & choice1) / N)
print("probability choice 2\n", np.sum(doors & choice2) / N)