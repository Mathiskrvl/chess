import './auth.css'

document.getElementById('login').innerHTML = `
<div class="center">
    <button id="show-login">Login</button>
</div>
<div class="popup">
    <div class="close-btn">&times;</div>
    <div class="form">
        <h2>Login</h2>
        <div class="form-element">
            <label for="email">Email</label>
            <input type="text" id="email" placeholder="Enter email">
        </div>
        <div class="form-element">
            <label for="password">Password</label>
            <input type="password" id="password" placeholder="Enter password">
        </div>
        <div class="form-element">
            <input type="checkbox" id="remember-me">
            <label for="remember-me">Remember me</label>
        </div>
        <div class="form-element">
            <button>Sign in</button>
        </div>
        <div class="form-element">
            <a href="#">Forgot password</a>
        </div>
    </div>
</div>
`

document.getElementById('show-login').addEventListener("click",() => {
    document.querySelector(".popup").classList.toggle("active")
})

document.querySelector('.close-btn').addEventListener("click",() => {
    document.querySelector(".popup").classList.remove("active")
})