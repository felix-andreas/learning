from tensorflow import keras
import numpy as np
from PIL import Image

image = (np.asarray(Image.open("test.png")).sum(2) // 3).reshape(1, 28, 28)

model = keras.models.load_model("mnist.h5")
prediction = model.predict(image)
print(np.argmax(prediction))
