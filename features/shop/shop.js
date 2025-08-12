// JavaScript for the shop page

document.addEventListener('DOMContentLoaded', () => {
    const products = [
        {
            id: 1,
            name: 'Labrador',
            price: '€35.00',
            imageUrl: 'assets/labrador.png',
            description: 'Une figurine élégante avec des lignes épurées, parfait pour une touche moderne.',
            shopifyLink: 'https://example.shopify.com/vase-geometrique' // Placeholder link
        },
        {
            id: 2,
            name: 'Berger Allemand',
            price: '€45.00',
            imageUrl: 'assets/berger.png',
            description: 'Support robuste et stylisé pour votre casque de jeu, personnalisable.',
            shopifyLink: 'https://example.shopify.com/support-casque' // Placeholder link
        },
        {
            id: 3,
            name: 'Cheval',
            price: '€60.00',
            imageUrl: 'assets/cheval.png',
            description: 'Une figurine de cheval, un chef-d\'œuvre de l\'impression 3D.',
            shopifyLink: 'https://example.shopify.com/dragon-articule' // Placeholder link
        }
    ];

    const productGrid = document.querySelector('.product-grid');

    if (productGrid) {
        products.forEach(product => {
            const productCard = document.createElement('div');
            productCard.classList.add('product-card');

            productCard.innerHTML = `
                <img src="${product.imageUrl}" alt="${product.name}">
                <h3>${product.name}</h3>
                <p>${product.description}</p>
                <p class="price">${product.price}</p>
                <a href="${product.shopifyLink}" target="_blank" class="buy-button">Acheter sur Shopify</a>
            `;
            productGrid.appendChild(productCard);
        });
    }
});
