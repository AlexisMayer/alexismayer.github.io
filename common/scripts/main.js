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

  const header = document.querySelector('.main-header');
  const headerHeight = header ? header.offsetHeight : 0;
  const elementTop = element.getBoundingClientRect().top + window.scrollY;
  const offset = 20; // 20px margin

  window.scrollTo({
    top: elementTop - headerHeight - offset,
    behavior: 'smooth'
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

// --- ROTATING TEXT EFFECT ---
document.addEventListener("DOMContentLoaded", () => {
  setScrollSnapType();
  window.addEventListener("resize", debounce(setScrollSnapType, 250));

  const rotatingText = document.getElementById("rotating-text");
  if (!rotatingText) return;

  const pageH1 = document.querySelector("h1")?.textContent;
  let phrases = [];

  if (pageH1 === "CraftData") {
    phrases = [
      "Développement sur mesure, Data et IA.",
      "Solutions Data, Machine Learning, LLM.",
      "Experts en Large Language Models (LLM) et RAG.",
      "Du concept à l'objet : nous donnons vie à vos idées.",
      "L'innovation au croisement du software et de la matière.",
    ];
  } else if (pageH1 === "Logiciel sur mesure") {
    phrases = [
      "Transformons vos données en levier de performance",
      "Des solutions sur mesure, conçues pour vos enjeux métier",
      "Un accompagnement durable pour faire évoluer vos modèles",
    ];
  } else {
    return; // Do nothing if the page is not recognized
  }

  if (phrases.length === 0) return;

  let index = 0;
  // Immediately set the first text
  rotatingText.textContent = phrases[index];

  setInterval(() => {
    index = (index + 1) % phrases.length;
    rotatingText.style.opacity = 0;

    setTimeout(() => {
      rotatingText.textContent = phrases[index];
      rotatingText.style.opacity = 1;
    }, 500);
  }, 4000);
});

// --- CARD EXPANSION & CAROUSEL LOGIC ---
let activeCarousel = null;

function toggleCard(card) {
  const isExpanded = card.classList.contains("expanded");
  const html = document.documentElement;

  document.querySelectorAll(".data-card.expanded").forEach((c) => {
    if (c !== card) {
      closeCard(c, false);
    }
  });

  const expandedContent = card.querySelector(".data-card-expanded-content");
  if (!expandedContent) return;

  if (!isExpanded) {
    const section = card.closest(".section");
    if (!section) return;

    html.style.scrollSnapType = "none";

    card.classList.add("expanded");
    section.querySelectorAll(".data-card").forEach((otherCard) => {
      if (otherCard !== card) {
        otherCard.classList.add("collapsed");
      }
    });

    // Set height dynamically
    expandedContent.style.maxHeight = expandedContent.scrollHeight + "px";

    if (card.id === "rag-use-case-card") {
      initCarousel(card);
    }

    setTimeout(() => {
      smoothScrollToElement(card);
    }, 300);
  } else {
    closeCard(card, true);
  }
}

function closeCard(card, reEnableSnap = true) {
  const section = card.closest(".section");
  if (!section) return;

  const expandedContent = card.querySelector(".data-card-expanded-content");
  if (expandedContent) {
    expandedContent.style.maxHeight = null;
  }

  card.classList.remove("expanded");

  section.querySelectorAll(".data-card.collapsed").forEach((otherCard) => {
    otherCard.classList.remove("collapsed");
  });

  if (reEnableSnap) {
    setScrollSnapType();
  }
}

function initCarousel(card) {
  const track = card.querySelector(".carousel-track");
  if (!track) return;

  const slides = Array.from(track.children);
  const nextButton = card.querySelector(".carousel-arrow.next");
  const prevButton = card.querySelector(".carousel-arrow.prev");
  let slideWidth = slides[0].getBoundingClientRect().width;
  let currentSlide = 0;

  const moveToSlide = (targetIndex) => {
    if (targetIndex < 0 || targetIndex >= slides.length) return;

    slideWidth = slides[0].getBoundingClientRect().width;
    track.style.transform = "translateX(-" + targetIndex * slideWidth + "px)";

    slides[currentSlide].classList.remove("active");
    slides[targetIndex].classList.add("active");
    currentSlide = targetIndex;

    updateArrows();
    handleSlideChange(slides[targetIndex]);
  };

  const updateArrows = () => {
    prevButton.style.display = currentSlide === 0 ? "none" : "block";
    nextButton.style.display =
      currentSlide === slides.length - 1 ? "none" : "block";
  };

  nextButton.addEventListener("click", (e) => {
    e.stopPropagation();
    moveToSlide(currentSlide + 1);
  });
  prevButton.addEventListener("click", (e) => {
    e.stopPropagation();
    moveToSlide(currentSlide - 1);
  });

  // Swipe functionality
  let touchStartX = 0;
  track.addEventListener(
    "touchstart",
    (e) => (touchStartX = e.changedTouches[0].screenX),
    { passive: true },
  );
  track.addEventListener("touchend", (e) => {
    const touchEndX = e.changedTouches[0].screenX;
    if (touchEndX < touchStartX - 50) moveToSlide(currentSlide + 1);
    if (touchEndX > touchStartX + 50) moveToSlide(currentSlide - 1);
  });

  // Simulation logic
  const handleSlideChange = (slide) => {
    const progressBar = card.querySelector(".progress-bar");
    const progressText = card.querySelector(".progress-text");
    if (!progressBar) return;

    progressBar.style.transition = "none";
    progressBar.style.width = "0%";
    progressText.textContent = "Fine-tuning en cours...";

    if (slide.contains(progressBar)) {
      setTimeout(() => {
        progressBar.style.transition = "width 2s ease-in-out";
        progressBar.style.width = "100%";
        progressText.textContent = "Fine-tuning terminé !";
      }, 500);
    }
  };

  // Initial setup
  moveToSlide(0);

  // Resize listener
  const resizeObserver = new ResizeObserver(
    debounce(() => {
      moveToSlide(currentSlide);
    }, 200),
  );
  resizeObserver.observe(card);
}

// --- SIMULATION LOGIC (FILE UPLOAD & CHAT) ---
document.addEventListener("DOMContentLoaded", () => {
  const fileInput = document.getElementById("pdf-upload");
  const fileInfo = document.querySelector(".file-info");
  if (fileInput) {
    fileInput.addEventListener("change", () => {
      fileInfo.textContent =
        fileInput.files.length > 0
          ? `${fileInput.files.length} fichier(s) sélectionné(s).`
          : "Aucun fichier sélectionné";
    });
  }

  const chatInput = document.querySelector(".chat-input");
  const chatSendButton = document.querySelector(".chat-send-button");
  const chatBox = document.querySelector(".chat-box");
  const chatContainer = document.querySelector(".chat-container");

  // Prevent card from closing when interacting with the chat
  if (chatContainer) {
    chatContainer.addEventListener("click", (e) => e.stopPropagation());
  }

  const handleSendMessage = () => {
    const message = chatInput.value.trim();
    if (message) {
      const userMsg = document.createElement("div");
      userMsg.className = "chat-message user";
      userMsg.textContent = message;
      chatBox.appendChild(userMsg);
      chatInput.value = "";
      chatBox.scrollTop = chatBox.scrollHeight;

      setTimeout(() => {
        const botMsg = document.createElement("div");
        botMsg.className = "chat-message bot";
        botMsg.textContent = "[Réponse IA simulée]";
        chatBox.appendChild(botMsg);
        chatBox.scrollTop = chatBox.scrollHeight;
      }, 1500);
    }
  };

  if (chatSendButton && chatInput) {
    chatSendButton.addEventListener("click", handleSendMessage);
    chatInput.addEventListener(
      "keypress",
      (e) => e.key === "Enter" && handleSendMessage(),
    );
  }
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
