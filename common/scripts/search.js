document.addEventListener('DOMContentLoaded', function() {
    const searchInput = document.getElementById('search-input');
    const searchSuggestions = document.getElementById('search-suggestions');

    const searchData = [
        {
            keyword: "Service d'impression 3D par IA",
            url: "features/products/index.html",
            content: "Donnez vie à vos idées. Décrivez simplement votre concept, notre IA génère le modèle 3D et nous l'imprimons pour vous. De l'imagination à l'objet réel, sans effort."
        },
        {
            keyword: "Nos produits et créations 3D",
            url: "features/products/index.html",
            content: "Découvrez notre service d'impression 3D révolutionnaire et notre ferme d'imprimantes. Nous transformons vos idées en réalité et soutenons la production locale."
        },
        {
            keyword: "Solutions Data & IA sur mesure",
            url: "features/services/index.html",
            content: "Nous transformons vos données en avantage concurrentiel. Découvrez nos services sur mesure : audit, stratégie, développement de modèles IA, MLOps et formation."
        },
        {
            keyword: "Boutique de créations 3D",
            url: "features/shop/index.html",
            content: "Explorez notre boutique de créations 3D uniques. Trouvez des figurines d'animaux, des objets décoratifs et des cadeaux originaux imprimés en 3D."
        },
        { keyword: "Contacter le groupe CraftData", url: "#contact-group", content: "Une question ? Un projet ? Contactez-nous pour discuter de partenariats, d'investissements ou pour toute autre demande." },
        { keyword: "Notre Vision : L'écosystème CraftData", url: "#vision-section", content: "Notre mission est de bâtir un écosystème d'innovation en Data et IA. Nous unissons des expertises pour développer des solutions de pointe et propulser des projets à fort potentiel." },
        { keyword: "Les entités du groupe CraftData", url: "#nos-entites", content: "Découvrez les entités qui forment notre écosystème : CraftData Services pour les solutions d'entreprise, PrintGenie pour l'impression 3D par IA, et notre Boutique de créations uniques." },
        { keyword: "Accueil - CraftData", url: "index.html", content: "Bienvenue chez CraftData, l'écosystème qui accélère l'innovation en Data et IA. Explorez nos services, nos produits et notre boutique." },
        {
            keyword: "Figurine de Labrador imprimée en 3D",
            url: "features/shop/index.html?product=1",
            content: "Adoptez cette élégante figurine de Labrador. Imprimée en 3D avec des lignes épurées, elle apportera une touche de modernité à votre intérieur. À partir de 29,90€."
        },
        {
            keyword: "Figurine de Berger Allemand imprimée en 3D",
            url: "features/shop/index.html?product=2",
            content: "Un support pour casque en forme de Berger Allemand. Robuste, stylisé et personnalisable, c'est l'accessoire parfait pour votre bureau de jeu. 45,00€."
        },
        {
            keyword: "Figurine de Cheval imprimée en 3D",
            url: "features/shop/index.html?product=3",
            content: "Admirez ce chef-d'œuvre de l'impression 3D : une magnifique figurine de cheval aux détails soignés. Une pièce maîtresse pour votre collection. 60,00€."
        },
        {
            keyword: "Créez votre propre objet 3D",
            url: "features/shop/index.html?product=4",
            content: "Votre imagination est la seule limite. Utilisez notre service d'impression 3D par IA pour donner vie à vos idées les plus folles. À partir de 29,90€."
        }
    ];

    if (searchInput && searchSuggestions) {
        searchInput.addEventListener('input', function() {
            const query = searchInput.value.toLowerCase();
            searchSuggestions.innerHTML = ''; // Clear previous suggestions

            if (query.length === 0) {
                searchSuggestions.style.display = 'none';
                return;
            }

            const filteredSuggestions = searchData.filter(item =>
                item.keyword.toLowerCase().includes(query) || item.content.toLowerCase().includes(query)
            );

            if (filteredSuggestions.length > 0) {
                filteredSuggestions.forEach(item => {
                    const suggestionItem = document.createElement('div');
                    suggestionItem.classList.add('search-suggestion-item');
                    suggestionItem.textContent = item.keyword; // Display keyword, but search on content
                    suggestionItem.dataset.url = item.url;
                    suggestionItem.addEventListener('click', function() {
                        window.location.href = this.dataset.url;
                    });
                    searchSuggestions.appendChild(suggestionItem);
                });
                searchSuggestions.style.display = 'block';
            } else {
                searchSuggestions.style.display = 'none';
            }
        });

        // Hide suggestions when clicking outside
        document.addEventListener('click', function(event) {
            if (!searchInput.contains(event.target) && !searchSuggestions.contains(event.target)) {
                searchSuggestions.style.display = 'none';
            }
        });
    }
});
