// Navigation scroll effect
window.addEventListener('scroll', () => {
  const nav = document.querySelector('nav');
  if (window.scrollY > 50) {
    nav.style.boxShadow = '0 4px 20px rgba(0,0,0,0.08)';
    nav.style.padding = '0.5rem 5vw';
  } else {
    nav.style.boxShadow = 'none';
    nav.style.padding = '0 5vw';
  }
});

// Mobile menu toggle
const navToggle = document.getElementById('navToggle');
const navLinks = document.getElementById('navLinks');

if (navToggle) {
  navToggle.addEventListener('click', () => {
    navLinks.classList.toggle('open');
    navToggle.classList.toggle('active');
  });
}

// Tabs handling (Universal)
document.querySelectorAll('.tab-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    const tabGroup = btn.parentElement;
    const tabId = btn.dataset.tab;
    
    // Remove active from buttons in this group
    tabGroup.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
    btn.classList.add('active');
    
    // Find the target content
    const target = document.getElementById('tab-' + tabId);
    if (target) {
      // Find all sibling content areas
      const contentGroup = target.parentElement;
      contentGroup.querySelectorAll('.usecase-content, .pricing-content').forEach(c => c.classList.remove('active'));
      target.classList.add('active');
    }
  });
});

// FAQ accordion
document.querySelectorAll('.faq-q').forEach(q => {
  q.addEventListener('click', () => {
    const item = q.parentElement;
    const wasOpen = item.classList.contains('open');
    document.querySelectorAll('.faq-item').forEach(i => i.classList.remove('open'));
    if (!wasOpen) item.classList.add('open');
  });
});

// Image Zoom Logic
const imageModal = document.getElementById('imageModal');
const modalImg = document.getElementById('modalImg');
const zoomableImages = document.querySelectorAll('.zoomable');

if (imageModal && modalImg) {
  zoomableImages.forEach(img => {
    img.addEventListener('click', () => {
      imageModal.classList.add('active');
      modalImg.src = img.src;
      document.body.style.overflow = 'hidden'; // Prevent scroll
    });
  });

  const closeModal = () => {
    imageModal.classList.remove('active');
    document.body.style.overflow = ''; // Restore scroll
  };

  imageModal.addEventListener('click', closeModal);
}
