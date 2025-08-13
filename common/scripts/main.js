// --- UTILITY FUNCTIONS ---
// Throttle function: limits how often a function can be called.
function throttle(func, limit) {
  let inThrottle;
  return function() {
    const args = arguments;
    const context = this;
    if (!inThrottle) {
      func.apply(context, args);
      inThrottle = true;
      setTimeout(() => inThrottle = false, limit);
    }
  }
}

// Debounce function: delays invoking a function until after a certain time has passed without it being called.
function debounce(func, delay) {
  let timeout;
  return function() {
    const context = this;
    const args = arguments;
    clearTimeout(timeout);
    timeout = setTimeout(() => func.apply(context, args), delay);
  };
}


// --- SCROLL PROGRESS BAR ---
const scrollProgress = document.querySelector(".scroll-progress");
const handleScroll = () => {
  const scrollTop = window.pageYOffset;
  const docHeight = document.body.offsetHeight - window.innerHeight;
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

      if (distSquared < 14400) { // 120 * 120
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
    if (rect.top <= window.innerHeight / 2 && rect.bottom >= window.innerHeight / 2) {
      currentSectionIndex = section.getAttribute("data-section");
    }
  });

  // Fallback for the very last section if it's not perfectly centered
  const lastSection = sections[sections.length - 1];
  if (lastSection) {
      const lastRect = lastSection.getBoundingClientRect();
      if (lastRect.bottom <= window.innerHeight + 10 && lastRect.bottom >= 0) { // If bottom of last section is visible
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
      targetSection.scrollIntoView({ behavior: "smooth" });
    }
  });
});


// --- ROTATING TEXT EFFECT ---
document.addEventListener("DOMContentLoaded", () => {
  document.documentElement.style.scrollSnapType = 'y mandatory';
  
  const rotatingText = document.getElementById("rotating-text");
  if (!rotatingText) return;

  const pageH1 = document.querySelector('h1')?.textContent;
  let phrases = [];

  if (pageH1 === 'CraftData') {
    phrases = [
      "Développement sur mesure, IA et impression 3D.",
      "Solutions Data, Machine Learning et fabrication additive.",
      "Experts en Large Language Models (LLM) et RAG.",
      "Du concept à l'objet : nous donnons vie à vos idées.",
      "L'innovation au croisement du software et de la matière.",
    ];
  } else if (pageH1 === 'Logiciel sur mesure') {
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

    document.querySelectorAll(".data-card.expanded").forEach(c => {
        if (c !== card) {
            closeCard(c, false);
        }
    });

    const expandedContent = card.querySelector('.data-card-expanded-content');
    if (!expandedContent) return;

    if (!isExpanded) {
        const section = card.closest(".section");
        if (!section) return;

        html.style.scrollSnapType = 'none';

        card.classList.add("expanded");
        section.querySelectorAll(".data-card").forEach(otherCard => {
            if (otherCard !== card) {
                otherCard.classList.add("collapsed");
            }
        });

        // Set height dynamically
        expandedContent.style.maxHeight = expandedContent.scrollHeight + "px";

        if (card.id === 'rag-use-case-card') {
            initCarousel(card);
        } else if (card.id === 'prediction-use-case-card') {
            initPredictionChart(card);
        }

        setTimeout(() => {
            const isMobile = window.innerWidth <= 768;
            if (isMobile) {
                card.scrollIntoView({ behavior: 'smooth', block: 'start' });
            } else {
                card.scrollIntoView({ behavior: 'smooth', block: 'center' });
            }
        }, 300);

    } else {
        closeCard(card, true);
    }
}

function closeCard(card, reEnableSnap = true) {
    const section = card.closest(".section");
    if (!section) return;

    const expandedContent = card.querySelector('.data-card-expanded-content');
    if (expandedContent) {
        expandedContent.style.maxHeight = null;
    }

    card.classList.remove("expanded");

    section.querySelectorAll(".data-card.collapsed").forEach(otherCard => {
        otherCard.classList.remove("collapsed");
    });

    if (reEnableSnap) {
        document.documentElement.style.scrollSnapType = 'y mandatory';
    }
}

function initCarousel(card) {
    const track = card.querySelector('.carousel-track');
    if (!track) return;

    const slides = Array.from(track.children);
    const nextButton = card.querySelector('.carousel-arrow.next');
    const prevButton = card.querySelector('.carousel-arrow.prev');
    let slideWidth = slides[0].getBoundingClientRect().width;
    let currentSlide = 0;

    const moveToSlide = (targetIndex) => {
        if (targetIndex < 0 || targetIndex >= slides.length) return;

        slideWidth = slides[0].getBoundingClientRect().width;
        track.style.transform = 'translateX(-' + targetIndex * slideWidth + 'px)';

        slides[currentSlide].classList.remove('active');
        slides[targetIndex].classList.add('active');
        currentSlide = targetIndex;

        updateArrows();
        handleSlideChange(slides[targetIndex]);
    }

    const updateArrows = () => {
        prevButton.style.display = (currentSlide === 0) ? 'none' : 'block';
        nextButton.style.display = (currentSlide === slides.length - 1) ? 'none' : 'block';
    }

    nextButton.addEventListener('click', (e) => {
        e.stopPropagation();
        moveToSlide(currentSlide + 1);
    });
    prevButton.addEventListener('click', (e) => {
        e.stopPropagation();
        moveToSlide(currentSlide - 1);
    });

    // Swipe functionality
    let touchStartX = 0;
    track.addEventListener('touchstart', e => touchStartX = e.changedTouches[0].screenX, { passive: true });
    track.addEventListener('touchend', e => {
        const touchEndX = e.changedTouches[0].screenX;
        if (touchEndX < touchStartX - 50) moveToSlide(currentSlide + 1);
        if (touchEndX > touchStartX + 50) moveToSlide(currentSlide - 1);
    });

    // Simulation logic
    const handleSlideChange = (slide) => {
        const progressBar = card.querySelector('.progress-bar');
        const progressText = card.querySelector('.progress-text');
        if (!progressBar) return;

        progressBar.style.transition = 'none';
        progressBar.style.width = '0%';
        progressText.textContent = 'Fine-tuning en cours...';

        if (slide.contains(progressBar)) {
            setTimeout(() => {
                progressBar.style.transition = 'width 2s ease-in-out';
                progressBar.style.width = '100%';
                progressText.textContent = 'Fine-tuning terminé !';
            }, 500);
        }
    }

    // Initial setup
    moveToSlide(0);

    // Resize listener
    const resizeObserver = new ResizeObserver(debounce(() => {
        moveToSlide(currentSlide);
    }, 200));
    resizeObserver.observe(card);
}

function initPredictionChart(card) {
    const ctx = card.querySelector('#predictionChart').getContext('2d');
    const labels = ['Jan', 'Fév', 'Mar', 'Avr', 'Mai', 'Juin', 'Juil', 'Août', 'Sep', 'Oct', 'Nov', 'Déc'];
    const historicalData = [65, 59, 80, 81, 56, 55, 40, 45, 50, 60, 70, 75];
    const predictedData = [null, null, null, null, null, null, null, null, 55, 65, 75, 80]; // Predictions start from September

    new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                {
                    label: 'Données Historiques',
                    data: historicalData,
                    borderColor: 'rgba(124, 58, 237, 1)',
                    backgroundColor: 'rgba(124, 58, 237, 0.2)',
                    fill: false,
                    tension: 0.1
                },
                {
                    label: 'Prévisions',
                    data: predictedData,
                    borderColor: 'rgba(0, 212, 255, 1)',
                    backgroundColor: 'rgba(0, 212, 255, 0.2)',
                    borderDash: [5, 5],
                    fill: false,
                    tension: 0.1
                }
            ]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            scales: {
                y: {
                    beginAtZero: true,
                    title: {
                        display: true,
                        text: 'Quantité'
                    }
                },
                x: {
                    title: {
                        display: true,
                        text: 'Mois'
                    }
                }
            },
            plugins: {
                legend: {
                    display: true,
                    position: 'top',
                },
                tooltip: {
                    mode: 'index',
                    intersect: false,
                }
            }
        }
    });
}

// --- SIMULATION LOGIC (FILE UPLOAD & CHAT) ---
document.addEventListener('DOMContentLoaded', () => {
    const fileInput = document.getElementById('pdf-upload');
    const fileInfo = document.querySelector('.file-info');
    if (fileInput) {
        fileInput.addEventListener('change', () => {
            fileInfo.textContent = fileInput.files.length > 0 ? `${fileInput.files.length} fichier(s) sélectionné(s).` : 'Aucun fichier sélectionné';
        });
    }

    const chatInput = document.querySelector('.chat-input');
    const chatSendButton = document.querySelector('.chat-send-button');
    const chatBox = document.querySelector('.chat-box');
    const chatContainer = document.querySelector('.chat-container');

    // Prevent card from closing when interacting with the chat
    if (chatContainer) {
        chatContainer.addEventListener('click', e => e.stopPropagation());
    }

    const handleSendMessage = () => {
        const message = chatInput.value.trim();
        if (message) {
            const userMsg = document.createElement('div');
            userMsg.className = 'chat-message user';
            userMsg.textContent = message;
            chatBox.appendChild(userMsg);
            chatInput.value = '';
            chatBox.scrollTop = chatBox.scrollHeight;

            setTimeout(() => {
                const botMsg = document.createElement('div');
                botMsg.className = 'chat-message bot';
                botMsg.textContent = '[Réponse IA simulée]';
                chatBox.appendChild(botMsg);
                chatBox.scrollTop = chatBox.scrollHeight;
            }, 1500);
        }
    }

    if (chatSendButton && chatInput) {
        chatSendButton.addEventListener('click', handleSendMessage);
        chatInput.addEventListener('keypress', e => e.key === 'Enter' && handleSendMessage());
    }
});


// --- INITIALIZATION ---
document.addEventListener('DOMContentLoaded', () => {
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
    const showDemoButton = document.querySelector('.show-demo-button');

    if (showDemoButton) {
        showDemoButton.addEventListener('click', (event) => {
            event.stopPropagation(); // Prevent the card from closing

            const card = showDemoButton.closest('.data-card');
            if (!card) return;

            const carouselContainer = card.querySelector('.carousel-container');
            const expandedContent = card.querySelector('.data-card-expanded-content');

            if (carouselContainer && expandedContent) {
                // Show the carousel and hide the button
                carouselContainer.style.display = 'block';
                showDemoButton.parentElement.style.display = 'none';

                // Recalculate the card's height to fit the new content
                expandedContent.style.maxHeight = expandedContent.scrollHeight + 'px';
            }
        });
    }
});

document.addEventListener('DOMContentLoaded', () => {
    const scrollIndicators = document.querySelectorAll('.scroll-indicator');

    scrollIndicators.forEach(indicator => {
        indicator.addEventListener('click', () => {
            const currentSection = indicator.closest('section');
            if (currentSection) {
                // Find the next section after the current section
                const nextSection = currentSection.nextElementSibling;
                if (nextSection) {
                    nextSection.scrollIntoView({ behavior: 'smooth' });
                }
            }
        });
    });
});

// Back to Top Button Logic
document.addEventListener('DOMContentLoaded', () => {
    const backToTopButton = document.getElementById('back-to-top');

    if (backToTopButton) {
        const toggleBackToTop = () => {
            if (window.pageYOffset > 200) { // Show button after scrolling 200px
                backToTopButton.classList.add('show');
            } else {
                backToTopButton.classList.remove('show');
            }
        };

        window.addEventListener('scroll', throttle(toggleBackToTop, 100)); // Use throttle for performance
        toggleBackToTop(); // Initial check

        backToTopButton.addEventListener('click', () => {
            window.scrollTo({
                top: 0,
                behavior: 'smooth'
            });
        });
    }

    // Set return_to parameter for login button
    const loginButton = document.querySelector('.header-login-button[href*="personal.html"]');
    if (loginButton) {
        const currentPagePath = window.location.pathname;
        const personalPagePath = loginButton.getAttribute('href');
        
        const returnToUrl = encodeURIComponent(currentPagePath);
        
        const separator = personalPagePath.includes('?') ? '&' : '?';
        loginButton.href = `${personalPagePath}${separator}return_to=${returnToUrl}`;
    }
});