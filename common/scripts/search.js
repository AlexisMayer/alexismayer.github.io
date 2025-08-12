document.addEventListener('DOMContentLoaded', function() {
    const searchInput = document.getElementById('search-input');
    const searchSuggestions = document.getElementById('search-suggestions');

    const searchData = [
        {
            keyword: "GPT 3D Printer",
            url: "features/products/index.html",
            content: "GPT 3D Printer Service d'impression 3D Transformez vos idées en objets réels avec l'IA. Comment ça marche ? Imaginez L'IA Crée Recevez Tu as une idée géniale mais pas les compétences en modélisation 3D ? Décris simplement ton objet en langage naturel. Notre IA comprend tes intentions. Ton concept prend forme, sans effort ni logiciel complexe. La modélisation 3D est longue et technique ? Notre intelligence artificielle génère un modèle 3D parfait en quelques secondes. Un fichier prêt à imprimer, sans les heures de travail habituelles. Tu n'as pas d'imprimante 3D ou tu ne sais pas l'utiliser ? Nous imprimons votre création en 3D et vous la livrons directement chez vous. Ton idée matérialisée, livrée directement chez toi. Nos Formules 1 Jeton 5 Jetons Offre Pro"
        },
        {
            keyword: "Produits",
            url: "features/products/index.html",
            content: "GPT 3D Printer Service d'impression 3D Transformez vos idées en objets réels avec l'IA. Comment ça marche ? Imaginez L'IA Crée Recevez Tu as une idée géniale mais pas les compétences en modélisation 3D ? Décris simplement ton objet en langage naturel. Notre IA comprend tes intentions. Ton concept prend forme, sans effort ni logiciel complexe. La modélisation 3D est longue et technique ? Notre intelligence artificielle génère un modèle 3D parfait en quelques secondes. Un fichier prêt à imprimer, sans les heures de travail habituelles. Tu n'as pas d'imprimante 3D ou tu ne sais pas l'utiliser ? Nous imprimons votre création en 3D et vous la livrons directement chez vous. Ton idée matérialisée, livrée directement chez toi. Nos Formules 1 Jeton 5 Jetons Offre Pro"
        },
        {
            keyword: "Services",
            url: "features/services/index.html",
            content: "Logiciel SaaS, CaaS, PaaS, CraftData Services Solutions IA & Données Votre partenaire en solutions Data & IA sur mesure Notre mission Transformer vos données en décisions, vos processus en solutions intelligentes, et vos idées en produits data-centrés. Data First Chaque projet commence par une compréhension fine de vos données. Cartographie & Audit Qualité & Fiabilité Potentiel d’usage Architecture Data IA sur mesure Pas de modèles génériques. Nous développons des solutions IA adaptées à vos contraintes, vos systèmes, vos objectifs. Modèles personnalisés Explicabilité & confiance Intégration métier Scalabilité & robustesse Évolution continue Nos solutions vivent, s’adaptent et s’améliorent avec vous. Monitoring intelligent MLOps & CI/CD Amélioration incrémentale Transfert & autonomie Nos Solutions Nos solutions combinent conseil, développement sur mesure et accompagnement long terme, pour faire de vos données un véritable moteur de performance. Audit & Stratégie Évaluation complète de votre écosystème data et définition d'une roadmap personnalisée Audit Data & IA Cadrage fonctionnel & technique Architecture cible Roadmap Stratégique Développement IA Création de modèles d'intelligence artificielle sur-mesure, du prototypage à la mise en production à grande échelle. Modèles IA sur mesure MLOps & pipelines IA Automatisation intelligente APIs et intégration Formation & Support Accompagnement de vos équipes avec des formations pratiques et un support technique continu Formation sur mesure Workshops pratiques Support technique continu Transfert de compétences Études de cas Voici deux cas concrets illustrant comment l’IA appliquée aux données permet d’apporter des gains de performance immédiats IA documentaire (LLM / RAG) Moteur de recherche intelligent sur documents internes Problématique Solution Résultat Prévision des approvisionnements Modèle de machine learning pour anticiper les besoins logistiques Problématique Solution Résultat Prêt à transformer vos données ? Contactez-nous dès aujourd'hui pour découvrir comment CraftData peut révolutionner votre approche de la donnée"
        },
        {
            keyword: "Boutique",
            url: "features/shop/index.html",
            content: "Boutique en ligne - CraftData Notre Boutique 3D Nos Créations Imprimées en 3D"
        },
        {
            keyword: "Shop",
            url: "features/shop/index.html",
            content: "Boutique en ligne - CraftData Notre Boutique 3D Nos Créations Imprimées en 3D"
        },
        { keyword: "Contact", url: "#contact-group", content: "Contactez le Groupe CraftData Pour toute question concernant le groupe, les partenariats ou les investissements, n'hésitez pas à nous contacter." },
        { keyword: "Vision", url: "#vision-section", content: "Notre Vision CraftData est la fondation qui unit et propulse des entités spécialisées dans la Data et l'Intelligence Artificielle. Notre mission est de créer un écosystème d'innovation où chaque entité excelle dans son domaine, tout en bénéficiant de la synergie et de l'expertise collective du groupe. Nous investissons dans des projets à fort potentiel, développons des solutions de pointe et accompagnons nos équipes vers l'excellence." },
        { keyword: "Entités", url: "#nos-entites", content: "Découvrez les piliers de l'écosystème CraftData. CraftData Services Votre partenaire expert en développement de solutions Data & IA sur mesure pour les entreprises. PrintGenie Notre startup innovante qui transforme vos idées en objets 3D réels grâce à l'IA. CraftData Boutique Découvrez nos créations uniques imprimées en 3D." },
        { keyword: "Accueil", url: "index.html", content: "CraftData L'écosystème qui propulse l'innovation Data & IA Discutons de votre projet Nos Offres Nos Services Nos Produits Notre Boutique" }
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
