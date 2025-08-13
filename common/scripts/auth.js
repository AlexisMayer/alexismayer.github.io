// common/scripts/auth.js

// Helper function to decode JWT
function decodeJwtResponse(token) {
    let base64Url = token.split('.')[1];
    let base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
    let jsonPayload = decodeURIComponent(atob(base64).split('').map(function(c) {
        return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
    }).join(''));
    return JSON.parse(jsonPayload);
}

// Function to handle redirection
function redirectToReturnUrl() {
    const returnUrl = sessionStorage.getItem('return_to_url');
    if (returnUrl) {
        sessionStorage.removeItem('return_to_url'); // Clean up
        window.location.href = decodeURIComponent(returnUrl);
    } else {
        window.location.href = 'index.html'; // Default redirect
    }
}

function handleCredentialResponse(response) {
    const id_token = response.credential;
    console.log("JWT ID token: " + id_token);

    const profile = decodeJwtResponse(id_token);
    console.log("User Profile:", profile);

    // Simulate successful login
    document.getElementById('g_id_signin').style.display = 'none'; // Hide Google button
    document.getElementById('user-info').style.display = 'flex'; // Show user info
    document.getElementById('user-name').textContent = profile.name;
    document.getElementById('user-email').textContent = profile.email;
    document.getElementById('user-picture').src = profile.picture;

    // Store info in localStorage for persistence (fictional)
    localStorage.setItem('loggedInUser', JSON.stringify(profile));

    // Redirect after successful login
    redirectToReturnUrl();
}

// Function to handle logout
function signOut() {
    google.accounts.id.disableAutoSelect(); // Disable One Tap for next login
    localStorage.removeItem('loggedInUser'); // Clear stored info
    alert("Déconnexion réussie !");
    redirectToReturnUrl(); // Redirect after logout
}

// Check for stored user on page load
window.onload = function () {
    // Store return_to URL if present
    const urlParams = new URLSearchParams(window.location.search);
    const returnToParam = urlParams.get('return_to');
    if (returnToParam) {
        sessionStorage.setItem('return_to_url', returnToParam);
    }

    // Set href for "Retour à l'accueil" button
    const backButton = document.getElementById('back-to-previous-page-button');
    if (backButton) {
        const storedReturnUrl = sessionStorage.getItem('return_to_url');
        if (storedReturnUrl) {
            backButton.href = decodeURIComponent(storedReturnUrl);
        }
        // If no storedReturnUrl, it defaults to index.html as set in HTML
    }

    google.accounts.id.initialize({
        client_id: "VOTRE_ID_CLIENT.apps.googleusercontent.com", // IMPORTANT: Remplacez par votre véritable ID client Google
        callback: handleCredentialResponse
    });

    const loggedInUser = localStorage.getItem('loggedInUser');
    if (loggedInUser) {
        const profile = JSON.parse(loggedInUser);
        document.getElementById('g_id_signin').style.display = 'none';
        document.getElementById('user-info').style.display = 'flex';
        document.getElementById('user-name').textContent = profile.name;
        document.getElementById('user-email').textContent = profile.email;
        document.getElementById('user-picture').src = profile.picture;
    } else {
        // Only render the button if not already logged in
        google.accounts.id.renderButton(
            document.getElementById("g_id_signin"),
            { theme: "outline", size: "large" }
        );
        document.getElementById('user-info').style.display = 'none'; // Ensure user info is hidden
    }
};