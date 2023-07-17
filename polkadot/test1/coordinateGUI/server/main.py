import matplotlib
matplotlib.use("agg")

import io
import base64
import numpy as np                 # v 1.19.2
from PIL import Image
import matplotlib.pyplot as plt    # v 3.3.2
from flask import Flask, jsonify
from flask_cors import CORS, cross_origin

MIN = -10
MAX = 10
image_path = 'graph.png'

app = Flask(__name__)
CORS(app, resources={r"/*": {"origins": "*"}})

@app.route('/', methods = ["GET", "POST"])
def root():
    return "welcome!"

@app.route('/getNewCoordinate', methods = ["GET"])
def getNewCoordinate():
    point = generateNewPoint()
    return jsonify({ 'x': point[0], 'y': point[1] })

def generateNewPoint():
    x = np.random.randint(MIN, MAX)
    y = np.random.randint(MIN, MAX)
    return (x, y)

def generateAndSaveGraph():
    # generate new random point
    point = generateNewPoint()
    print('point:', point)
    # Enter x and y coordinates of points and colors
    xs = [point[0]]
    ys = [point[1]]
    # color red
    colors = ['r']
    # Select length of axes and the space between tick labels
    xmin, xmax, ymin, ymax = MIN, MAX, MIN, MAX
    ticks_frequency = 1
    # Plot points
    fig, ax = plt.subplots(figsize=(10, 10))
    ax.scatter(xs, ys, c=colors)
    # Draw lines connecting points to axes
    for x, y, c in zip(xs, ys, colors):
        ax.plot([x, x], [0, y], c=c, ls='--', lw=1.5, alpha=0.5)
        ax.plot([0, x], [y, y], c=c, ls='--', lw=1.5, alpha=0.5)
    
    # Set identical scales for both axes
    ax.set(xlim=(xmin-1, xmax+1), ylim=(ymin-1, ymax+1), aspect='equal')
    # Set bottom and left spines as x and y axes of coordinate system
    ax.spines['bottom'].set_position('zero')
    ax.spines['left'].set_position('zero')
    # Remove top and right spines
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    # Create 'x' and 'y' labels placed at the end of the axes
    ax.set_xlabel('x', size=14, labelpad=-24, x=1.03)
    ax.set_ylabel('y', size=14, labelpad=-21, y=1.02, rotation=0)
    # Create custom major ticks to determine position of tick labels
    x_ticks = np.arange(xmin, xmax+1, ticks_frequency)
    y_ticks = np.arange(ymin, ymax+1, ticks_frequency)
    ax.set_xticks(x_ticks[x_ticks != 0])
    ax.set_yticks(y_ticks[y_ticks != 0])
    # Create minor ticks placed at each integer to enable drawing of minor grid
    # lines: note that this has no effect in this example with ticks_frequency=1
    ax.set_xticks(np.arange(xmin, xmax+1), minor=True)
    ax.set_yticks(np.arange(ymin, ymax+1), minor=True)
    # Draw major and minor grid lines
    ax.grid(which='both', color='grey', linewidth=1, linestyle='-', alpha=0.2)
    # Draw arrows
    arrow_fmt = dict(markersize=4, color='black', clip_on=False)
    ax.plot((1), (0), marker='>', transform=ax.get_yaxis_transform(), **arrow_fmt)
    ax.plot((0), (1), marker='^', transform=ax.get_xaxis_transform(), **arrow_fmt)
    fig = ax.get_figure()
    # save the figure as png in current folder
    fig.savefig(image_path) 

@app.route('/getNewGraph', methods = ["GET"])
def getGraph():
    generateAndSaveGraph()
    print('image saved..')
    image = get_encoded_img()
    return jsonify({'image_url': image})

def get_encoded_img():
    img = Image.open(image_path, mode='r')
    img_byte_arr = io.BytesIO()
    img.save(img_byte_arr, format='PNG')
    my_encoded_img = base64.encodebytes(img_byte_arr.getvalue()).decode('ascii')
    return my_encoded_img

if __name__ == '__main__':
    app.run(port=8448, debug = True)

