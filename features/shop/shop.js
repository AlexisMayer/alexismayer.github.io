// JavaScript for the shop page

document.addEventListener('DOMContentLoaded', () => {
    const products = [
        {
            id: 1,
            name: 'Labrador',
            price: 'A partir de 29,90€',
            imageUrl: 'assets/labrador.png',
            description: 'Une figurine élégante avec des lignes épurées, parfait pour une touche moderne.',
            shopifyLink: 'https://example.shopify.com/vase-geometrique' // Placeholder link
        },
        {
            id: 3,
            name: 'Cheval',
            price: 'A partir de 29,90€',
            imageUrl: 'assets/cheval.png',
            description: 'Une figurine de cheval, avec des traits réalistes et des lignes fines.',
            shopifyLink: 'https://example.shopify.com/dragon-articule' // Placeholder link
        },
        {
            id: 4,
            name: 'Créez le vôtre',
            price: 'A partir de 29,90€',
            imageUrl: 'assets/berger.png',
            description: 'Grâce à notre produit GPT 3D Printer, donnez vie à vos idées les plus folles.',
            shopifyLink: '../products/index.html'
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
                ${product.id === 4 ?
                    `<a href="${product.shopifyLink}" class="buy-button">Découvrir</a>` :
                    `<a href="${product.shopifyLink}" target="_blank" class="buy-button">Acheter sur Shopify</a>`
                }
            `;
            productCard.id = `product-${product.id}`; // Add this line
            productGrid.appendChild(productCard);
        });

        // Scroll to product if ID is in URL
        const urlParams = new URLSearchParams(window.location.search);
        const productId = urlParams.get('product');
        if (productId) {
            const targetProductCard = document.getElementById(`product-${productId}`);
            if (targetProductCard) {
                targetProductCard.scrollIntoView({ behavior: 'smooth', block: 'center' });
                targetProductCard.classList.add('highlight'); // Optional: add a highlight class
                setTimeout(() => {
                    targetProductCard.classList.remove('highlight');
                }, 2000); // Remove highlight after 2 seconds
            }
        }
    }
});
