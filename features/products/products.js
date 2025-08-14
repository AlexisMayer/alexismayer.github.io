function toggleCard(card) {
    card.classList.toggle('expanded');
    const expandedContent = card.querySelector('.data-card-expanded-content');
    if (card.classList.contains('expanded')) {
        expandedContent.style.maxHeight = expandedContent.scrollHeight + 'px';
    } else {
        expandedContent.style.maxHeight = '0';
    }
}

function closeCard(card) {
    card.classList.remove('expanded');
    const expandedContent = card.querySelector('.data-card-expanded-content');
    expandedContent.style.maxHeight = '0';
}

document.addEventListener('DOMContentLoaded', () => {
    const showGalleryBtn = document.getElementById('show-gallery-btn');
    const closeGalleryBtn = document.getElementById('close-gallery-btn');
    const galleryOverlay = document.getElementById('product-gallery');
    const galleryImages = document.getElementById('gallery-images');

    const images = ['berger.png', 'cheval.png', 'labrador.png'];

    showGalleryBtn.addEventListener('click', () => {
        galleryImages.innerHTML = ''; // Clear previous images
        images.forEach(imageName => {
            const img = document.createElement('img');
            img.src = `assets/${imageName}`;
            img.alt = imageName.split('.')[0];
            galleryImages.appendChild(img);
        });
        galleryOverlay.style.display = 'flex';
    });

    closeGalleryBtn.addEventListener('click', () => {
        galleryOverlay.style.display = 'none';
    });

    galleryOverlay.addEventListener('click', (e) => {
        if (e.target === galleryOverlay) {
            galleryOverlay.style.display = 'none';
        }
    });
});