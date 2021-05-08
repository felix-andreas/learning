import cv2
import numpy as np

capture = cv2.VideoCapture("capture.mp4")
frame_size = (int(capture.get(3)), int(capture.get(4)))
skip = cv2.VideoWriter("skip.avi", cv2.VideoWriter_fourcc(*"MJPG"), 60, frame_size)
blur = cv2.VideoWriter("blur.avi", cv2.VideoWriter_fourcc(*"MJPG"), 60, frame_size)

n = 4
frames = np.empty((n, 1080, 1920, 3), dtype=np.uint8)
for i, (success, frame) in enumerate(iter(capture.read, (False, None))):
    frames[i % n] = frame
    if i % n == n - 1:
        skip.write(frames[0])
        blur.write(
            (frames[0] >> 2) + (frames[1] >> 2) + (frames[2] >> 2) + (frames[3] >> 2)
        )
