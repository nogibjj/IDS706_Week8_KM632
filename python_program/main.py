from PIL import Image, ImageFilter
import os
import psutil
import time

start_time = time.time_ns()

# IO operation
directory = "/workspaces/IDS706_Week8_KM632/input"
filename = "input.jpeg"
image_path = os.path.join(directory, filename)
image = Image.open(image_path)

# edit the image
new_size = (300, 200) 
resized_image = image.resize(new_size)
blurred_image = resized_image.filter(ImageFilter.GaussianBlur(radius=5))
blurred_image.save("output.jpeg")

# IO operation
image.close()
resized_image.close()
blurred_image.close()

end_time = time.time_ns()
execution_time_ns = end_time - start_time
print(f"Execution Time: {execution_time_ns} nanoseconds")
cpu_percent = psutil.cpu_percent()
print(f"CPU Utilization: {cpu_percent}%")