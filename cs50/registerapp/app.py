from flask import Flask, render_template, request, redirect

app = Flask(__name__)

students = []

@app.route('/')
def index():
    name = request.args.get("name")
    return render_template('index.html')

@app.route('/registrants')
def registerd():
    return render_template("registrants.html", students=students)

@app.route('/register', methods=["POST"])
def register():
    name = request.form.get("name")
    dorm = request.form.get("dorm")
    print(name, dorm)
    if not name or not dorm:
        return render_template("failure.html")
    students.append(f"{name} from {dorm}")
    return redirect("/registrants")
