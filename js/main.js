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
    const sectionTop = section.offsetTop;
    if (window.pageYOffset >= sectionTop - window.innerHeight / 2) {
      currentSectionIndex = section.getAttribute("data-section");
    }
  });

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
  const phrases = [
    "Transformons vos données en levier de performance",
    "Des solutions sur mesure, conçues pour vos enjeux métier",
    "Un accompagnement durable pour faire évoluer vos modèles",
  ];

  let index = 0;
  const rotatingText = document.getElementById("rotating-text");

  setInterval(() => {
    index = (index + 1) % phrases.length;
    rotatingText.style.opacity = 0;

    setTimeout(() => {
      rotatingText.textContent = phrases[index];
      rotatingText.style.opacity = 1;
    }, 500);
  }, 4000);
});


// --- CARD EXPANSION FUNCTIONALITY ---
function toggleCard(card) {
    const isExpanded = card.classList.contains("expanded");
    const html = document.documentElement;

    document.querySelectorAll(".data-card.expanded").forEach(c => {
        if (c !== card) {
            closeCard(c, false);
        }
    });

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

    card.classList.remove("expanded");

    section.querySelectorAll(".data-card.collapsed").forEach(otherCard => {
        otherCard.classList.remove("collapsed");
    });

    if (reEnableSnap) {
        document.documentElement.style.scrollSnapType = 'y mandatory';
    }
}


// --- INITIALIZATION ---
updateActiveDot();