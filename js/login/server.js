import dotenv from 'dotenv'
dotenv.config()
// if (process.env.NODE_ENV !== "production") {
//     import {
//         config
//     } from "dotenv";
//     config();
// }

import express from 'express';
import flash from 'express-flash';
import session from 'express-session';
import bcrypt from 'bcrypt';
import passport from 'passport';
import passport_local from 'passport-local';

passport.use(new passport_local.Strategy({
        usernameField: "email"
    },
    async (email, password, done) => {
        const user = users.find(user => user.email === email);
        if (user == null) {
            return done(null, false, {
                message: "No user with that email"
            });
        }

        try {
            if (await bcrypt.compare(password, user.hashedPassword)) {
                return done(null, user);
            } else {
                return done(null, false, {
                    message: "Wrong Password!"
                });
            }

        } catch (e) {
            return done(e);
        }
    }
));
passport.serializeUser((user, done) => done(null, user.id));
passport.deserializeUser((id, done) => done(null, users.find(user => user.id === id)));


const users = [];

const app = express();
app.set("view-engine", "ejs");
app.use(express.urlencoded({
    extended: false
}));
app.use(flash());
app.use(session({
    secret: process.env.SESSION_SECRET,
    resave: false,
    saveUninitialized: false
}));
app.use(passport.initialize());
app.use(passport.session());

app.get('/', checkAuthenticated, (request, response) => {
    response.render("index.ejs", {
        name: request.user.name
    });
});

app.get('/signup', checkNotAuthenticated, (request, response) => {
    response.render("signup.ejs");
});

app.post('/signup', async (request, response) => {
    try {
        const hashedPassword = await bcrypt.hash(request.body.password, 10);
        users.push({
            id: Date.now().toString(),
            name: request.body.name,
            email: request.body.email,
            hashedPassword
        });
        console.log("NEW USER", users[users.length - 1]);
        response.redirect("/login");
    } catch {
        response.redirect("/signup");
    }
});

app.get("/login", checkNotAuthenticated, (request, response) => response.render("login.ejs"));

app.post('/login', passport.authenticate("local", {
    successRedirect: "/",
    failureRedirect: '/login',
    failureFlash: true
}));

app.post("/logout", (request, response) => {
    request.logOut();
    response.redirect("/login");
})

function checkAuthenticated(request, response, next) {
    if (request.isAuthenticated()) {
        next();
    } else {
        response.redirect("/login");
    }
}

function checkNotAuthenticated(request, response, next) {
    if (request.isAuthenticated()) {
        response.redirect("/");
    } else {
        next();
    }
}


app.use(express.static('public'))
app.listen(3000);