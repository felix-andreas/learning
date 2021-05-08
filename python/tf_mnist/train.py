import tensorflow as tf
from tensorflow import keras

(images, labels), (test_images, test_labels) = keras.datasets.mnist.load_data()
breakpoint()

model = keras.Sequential(
    [
        keras.layers.Flatten(input_shape=(28, 28)),
        keras.layers.Dense(128, activation=tf.nn.relu),
        keras.layers.Dense(128, activation=tf.nn.relu),
        keras.layers.Dense(128, activation=tf.nn.relu),
        keras.layers.Dense(10, activation=tf.nn.softmax),
    ]
)

model.compile(
    optimizer=tf.keras.optimizers.Adam(),
    loss="sparse_categorical_crossentropy",
    metrics=["accuracy"],
)

model.fit(images, labels, epochs=5)

test_loss, test_acc = model.evaluate(test_images, test_labels)
print(f"test accuracy: {test_acc}")

predictions = model.predict(test_images)

model.save("mnist.h5")
