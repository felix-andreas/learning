from flask import Flask, render_template, request, jsonify

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('base.html')

@app.route('/search')
def search():
    q = request.args.get("q")
    with open("words.csv", 'r') as fh:
        all_words = fh.readlines()
    words = [word for word in all_words if q and word.startswith(q)]
    return jsonify(words)
