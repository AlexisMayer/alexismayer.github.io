// JavaScript for the shop page

document.addEventListener('DOMContentLoaded', () => {
    const products = [
        {
            id: 1,
            name: 'Vase Géométrique Minimaliste',
            price: '€35.00',
            imageUrl: 'assets/vase_geometrique.jpg',
            description: 'Un vase élégant avec des lignes épurées, parfait pour une touche moderne.',
            shopifyLink: 'https://example.shopify.com/vase-geometrique' // Placeholder link
        },
        {
            id: 2,
            name: 'Support Casque Gaming Personnalisé',
            price: '€45.00',
            imageUrl: 'assets/support_casque.jpg',
            description: 'Support robuste et stylisé pour votre casque de jeu, personnalisable.',
            shopifyLink: 'https://example.shopify.com/support-casque' // Placeholder link
        },
        {
            id: 3,
            name: 'Figurine Dragon Articulée',
            price: '€60.00',
            imageUrl: 'assets/dragon_articule.jpg',
            description: 'Une figurine de dragon entièrement articulée, un chef-d\'œuvre de l\'impression 3D.',
            shopifyLink: 'https://example.shopify.com/dragon-articule' // Placeholder link
        },
        {
            id: 4,
            name: 'Pot de Fleurs Suspendu Design',
            price: '€28.00',
            imageUrl: 'assets/pot_fleurs.jpg',
            description: 'Un pot de fleurs unique pour égayer votre intérieur ou extérieur.',
            shopifyLink: 'https://example.shopify.com/pot-fleurs' // Placeholder link
        },
        {
            id: 5,
            name: 'Organisateur de Bureau Modulaire',
            price: '€30.00',
            imageUrl: 'assets/organisateur_bureau.jpg',
            description: 'Gardez votre espace de travail ordonné avec cet organisateur personnalisable.',
            shopifyLink: 'https://example.shopify.com/organisateur-bureau' // Placeholder link
        },
        {
            id: 6,
            name: 'Lampe de Chevet Lune 3D',
            price: '€55.00',
            imageUrl: 'assets/lampe_lune.jpg',
            description: 'Une reproduction réaliste de la lune, parfaite pour une ambiance douce.',
            shopifyLink: 'https://example.shopify.com/lampe-lune' // Placeholder link
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
