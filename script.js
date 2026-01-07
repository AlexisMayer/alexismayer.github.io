// --- UTILITY FUNCTIONS ---
// Throttle function: limits how often a function can be called.
function throttle(func, limit) {
  let inThrottle;
  return function () {
    const args = arguments;
    const context = this;
    if (!inThrottle) {
      func.apply(context, args);
      inThrottle = true;
      setTimeout(() => (inThrottle = false), limit);
    }
  };
}

// Debounce function: delays invoking a function until after a certain time has passed without it being called.
function debounce(func, delay) {
  let timeout;
  return function () {
    const context = this;
    const args = arguments;
    clearTimeout(timeout);
    timeout = setTimeout(() => func.apply(context, args), delay);
  };
}

// Helper function to check if the device is mobile based on screen width
function isMobile() {
  return window.innerWidth <= 768; // Or any breakpoint you define as mobile
}

// Function to set scroll-snap-type based on device
function setScrollSnapType() {
  if (isMobile()) {
    document.documentElement.style.scrollSnapType = "none";
  } else {
    document.documentElement.style.scrollSnapType = "y mandatory";
  }
}

function smoothScrollToElement(element) {
  if (!element) return;

  const header = document.querySelector(".main-header");
  const headerHeight = header ? header.offsetHeight : 0;
  const elementTop = element.getBoundingClientRect().top + window.scrollY;
  const offset = 20; // 20px margin

  window.scrollTo({
    top: elementTop - headerHeight - offset,
    behavior: "smooth",
  });
}

// --- SCROLL PROGRESS BAR ---
const scrollProgress = document.querySelector(".scroll-progress");
const handleScroll = () => {
  const scrollTop = window.pageYOffset;
  const docHeight = document.documentElement.scrollHeight - window.innerHeight;
  const scrollPercent = scrollTop / docHeight;
  scrollProgress.style.transform = `scaleX(${scrollPercent})`;
};

window.addEventListener("scroll", throttle(handleScroll, 10));

// --- ANIMATED BACKGROUND CANVAS ---
const canvas = document.querySelector(".bg-canvas");
const ctx = canvas.getContext("2d");

function resizeCanvas() {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}

resizeCanvas();

window.addEventListener("resize", debounce(resizeCanvas, 250));

// --- PARTICLES SYSTEM ---
const particles = [];
for (let i = 0; i < 80; i++) {
  particles.push({
    x: Math.random() * canvas.width,
    y: Math.random() * canvas.height,
    size: Math.random() * 3 + 0.5,
    speedX: (Math.random() - 0.5) * 1.5,
    speedY: (Math.random() - 0.5) * 1.5,
    opacity: Math.random() * 0.6 + 0.2,
    color: Math.random() > 0.5 ? "rgba(124, 58, 237, " : "rgba(0, 212, 255, ",
  });
}

function animateParticles() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  particles.forEach((particle) => {
    particle.x += particle.speedX;
    particle.y += particle.speedY;

    if (particle.x > canvas.width) particle.x = 0;
    if (particle.x < 0) particle.x = canvas.width;
    if (particle.y > canvas.height) particle.y = 0;
    if (particle.y < 0) particle.y = canvas.height;

    ctx.beginPath();
    ctx.arc(particle.x, particle.y, particle.size, 0, Math.PI * 2);
    ctx.fillStyle = particle.color + particle.opacity + ")";
    ctx.fill();

    ctx.shadowBlur = 20;
    ctx.shadowColor = particle.color + "0.5)";
    ctx.fill();
    ctx.shadowBlur = 0;
  });

  particles.forEach((particle, i) => {
    particles.slice(i + 1).forEach((otherParticle) => {
      const dx = particle.x - otherParticle.x;
      const dy = particle.y - otherParticle.y;
      const distSquared = dx * dx + dy * dy;

      if (distSquared < 14400) {
        // 120 * 120
        const distance = Math.sqrt(distSquared);
        const opacity = (1 - distance / 120) * 0.3;
        const gradient = ctx.createLinearGradient(
          particle.x,
          particle.y,
          otherParticle.x,
          otherParticle.y,
        );
        gradient.addColorStop(0, particle.color + opacity + ")");
        gradient.addColorStop(1, otherParticle.color + opacity + ")");

        ctx.beginPath();
        ctx.moveTo(particle.x, particle.y);
        ctx.lineTo(otherParticle.x, otherParticle.y);
        ctx.strokeStyle = gradient;
        ctx.lineWidth = 1;
        ctx.stroke();
      }
    });
  });

  requestAnimationFrame(animateParticles);
}

animateParticles();

// --- SCROLL ANIMATIONS ---
const observerOptions = {
  threshold: 0.3,
  rootMargin: "0px 0px -50px 0px",
};

const observer = new IntersectionObserver((entries) => {
  entries.forEach((entry) => {
    if (entry.isIntersecting) {
      entry.target.querySelector(".section-content").classList.add("visible");
    }
  });
}, observerOptions);

document.querySelectorAll(".section").forEach((section) => {
  if (!section.classList.contains("hero")) {
    observer.observe(section);
  }
});

// --- NAVIGATION DOTS ---
const navDots = document.querySelectorAll(".nav-dot");
const sections = document.querySelectorAll("section[data-section]");

const updateActiveDot = () => {
  let currentSectionIndex = "0";
  sections.forEach((section) => {
    const rect = section.getBoundingClientRect();
    // A section is active if its top is within the top half of the viewport
    // or if it's the last section and scrolled to the bottom
    if (
      rect.top <= window.innerHeight / 2 &&
      rect.bottom >= window.innerHeight / 2
    ) {
      currentSectionIndex = section.getAttribute("data-section");
    }
  });

  // Fallback for the very last section if it's not perfectly centered
  const lastSection = sections[sections.length - 1];
  if (lastSection) {
    const lastRect = lastSection.getBoundingClientRect();
    if (lastRect.bottom <= window.innerHeight + 10 && lastRect.bottom >= 0) {
      // If bottom of last section is visible
      currentSectionIndex = lastSection.getAttribute("data-section");
    }
  }

  navDots.forEach((dot) => {
    dot.classList.toggle(
      "active",
      dot.getAttribute("data-section") === currentSectionIndex,
    );
  });
};

window.addEventListener("scroll", throttle(updateActiveDot, 100));

navDots.forEach((dot) => {
  dot.addEventListener("click", () => {
    const sectionIndex = dot.getAttribute("data-section");
    const targetSection = document.querySelector(
      `section[data-section="${sectionIndex}"]`,
    );
    if (targetSection) {
      smoothScrollToElement(targetSection);
    }
  });
});

// --- INITIALIZATION ---
document.addEventListener("DOMContentLoaded", () => {
  // Initialize navigation dots after DOM is fully loaded
  updateActiveDot();

  // Attach click listeners to nav dots
  navDots.forEach((dot) => {
    dot.addEventListener("click", () => {
      const sectionIndex = dot.getAttribute("data-section");
      const targetSection = document.querySelector(
        `section[data-section="${sectionIndex}"]`,
      );
      if (targetSection) {
        targetSection.scrollIntoView({ behavior: "smooth" });
      }
    });
  });

  // Show demo button logic
  const showDemoButton = document.querySelector(".show-demo-button");

  if (showDemoButton) {
    showDemoButton.addEventListener("click", (event) => {
      event.stopPropagation(); // Prevent the card from closing

      const card = showDemoButton.closest(".data-card");
      if (!card) return;

      const carouselContainer = card.querySelector(".carousel-container");
      const expandedContent = card.querySelector(".data-card-expanded-content");

      if (carouselContainer && expandedContent) {
        // Show the carousel and hide the button
        carouselContainer.style.display = "block";
        showDemoButton.parentElement.style.display = "none";

        // Recalculate the card's height to fit the new content
        expandedContent.style.maxHeight = expandedContent.scrollHeight + "px";
      }
    });
  }
});

document.addEventListener("DOMContentLoaded", () => {
  const scrollIndicators = document.querySelectorAll(".scroll-indicator");

  scrollIndicators.forEach((indicator) => {
    indicator.addEventListener("click", () => {
      const currentSection = indicator.closest("section");
      if (currentSection) {
        // Find the next section after the current section
        const nextSection = currentSection.nextElementSibling;
        if (nextSection) {
          nextSection.scrollIntoView({ behavior: "smooth" });
        }
      }
    });
  });
});

// Back to Top Button Logic
document.addEventListener("DOMContentLoaded", () => {
  const backToTopButton = document.getElementById("back-to-top");

  if (backToTopButton) {
    const toggleBackToTop = () => {
      if (window.pageYOffset > 200) {
        // Show button after scrolling 200px
        backToTopButton.classList.add("show");
      } else {
        backToTopButton.classList.remove("show");
      }
    };

    window.addEventListener("scroll", throttle(toggleBackToTop, 100)); // Use throttle for performance
    toggleBackToTop(); // Initial check

    backToTopButton.addEventListener("click", () => {
      window.scrollTo({
        top: 0,
        behavior: "smooth",
      });
    });
  }
});

// --- HEADER DROPDOWN MENU (CLICK-BASED) ---
document.addEventListener("DOMContentLoaded", function () {
  const dropdownToggle = document.querySelector(".dropdown .dropdown-toggle");
  const dropdownMenu = document.querySelector(".dropdown .dropdown-menu");

  if (dropdownToggle && dropdownMenu) {
    dropdownToggle.addEventListener("click", function (event) {
      event.stopPropagation(); // Empêche le clic de se propager à la fenêtre
      dropdownMenu.classList.toggle("show");
    });
  }

  // Ajoute un écouteur sur la fenêtre pour fermer le menu en cliquant à l'extérieur
  window.addEventListener("click", function (event) {
    const dropdownMenu = document.querySelector(".dropdown .dropdown-menu");
    // Si le menu déroulant existe, est affiché, et que le clic est en dehors de celui-ci
    if (dropdownMenu && dropdownMenu.classList.contains("show")) {
      const dropdown = dropdownMenu.closest(".dropdown");
      if (dropdown && !dropdown.contains(event.target)) {
        dropdownMenu.classList.remove("show");
      }
    }
  });
});

// --- THEME TOGGLE LOGIC ---
document.addEventListener("DOMContentLoaded", () => {
  const themeToggleBtn = document.getElementById("theme-toggle");
  const body = document.body;

  // Function to apply the theme
  function applyTheme(theme) {
    if (theme === "dark") {
      body.classList.remove("light-mode");
      themeToggleBtn.innerHTML =
        '<svg class="theme-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></svg><span>Thème</span>'; // Sun icon
      localStorage.setItem("theme", "dark");
    } else {
      body.classList.add("light-mode");
      themeToggleBtn.innerHTML =
        '<svg class="theme-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg><span>Thème</span>'; // Moon icon
      localStorage.setItem("theme", "light");
    }
  }

  // Function to toggle the theme
  function toggleTheme() {
    const currentTheme =
      localStorage.getItem("theme") ||
      (body.classList.contains("light-mode") ? "light" : "dark");
    if (currentTheme === "dark") {
      applyTheme("light");
    } else {
      applyTheme("dark");
    }
  }

  // Apply saved theme or system preference on load
  const savedTheme = localStorage.getItem("theme");
  if (savedTheme) {
    applyTheme(savedTheme);
  } else if (
    window.matchMedia &&
    window.matchMedia("(prefers-color-scheme: light)").matches
  ) {
    applyTheme("light"); // Default to light mode if system preference is light
  } else {
    applyTheme("dark"); // Default to dark mode
  }

  // Event listener for the toggle button
  if (themeToggleBtn) {
    themeToggleBtn.addEventListener("click", toggleTheme);
  }
});
