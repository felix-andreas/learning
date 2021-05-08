import pyautogui
import matplotlib.pyplot as plt
import numpy as np
from itertools import count

y = 120
from_, to = 190,250

for i in count():
    img = pyautogui.screenshot(region=(650, 200, 600, 150))
    data = np.array(img).mean(axis=2)
    # plt.imshow(img)
    # plt.show()
    # breakpoint()
    # print(data[120, 150:250])
    window = data[120, from_:to]
    jump = np.any(window != 255)
    # print(window)
    print(i, "jump:", jump)
    if jump:
        pyautogui.keyDown('up')
        pyautogui.keyUp('up')

