document.addEventListener("DOMContentLoaded", function () {
  const searchInput = document.getElementById("search-input");
  const searchSuggestions = document.getElementById("search-suggestions");

  const searchData = [
    {
      keyword: "Service d'impression 3D par IA",
      url: "features/products/index.html",
      content:
        "Donnez vie à vos idées. Décrivez simplement votre concept, notre IA génère le modèle 3D et nous l'imprimons pour vous. De l'imagination à l'objet réel, sans effort.",
    },
    {
      keyword: "Solutions Data & IA sur mesure",
      url: "features/services/index.html",
      content:
        "Nous transformons vos données en avantage concurrentiel. Découvrez nos services sur mesure : audit, stratégie, développement de modèles IA, MLOps et formation.",
    },
    {
      keyword: "Contacter le groupe CraftData",
      url: "#contact-group",
      content:
        "Une question ? Un projet ? Contactez-nous pour discuter de partenariats, d'investissements ou pour toute autre demande.",
    },
    {
      keyword: "Notre Vision : L'écosystème CraftData",
      url: "#vision-section",
      content:
        "Notre mission est de bâtir un écosystème d'innovation en Data et IA. Nous unissons des expertises pour développer des solutions de pointe et propulser des projets à fort potentiel.",
    },
    {
      keyword: "Les entités du groupe CraftData",
      url: "#nos-entites",
      content:
        "Découvrez les entités qui forment notre écosystème : CraftData Services pour les solutions d'entreprise, PrintGenie pour l'impression 3D par IA, et notre Boutique de créations uniques.",
    },
    {
      keyword: "Accueil - CraftData",
      url: "index.html",
      content:
        "Bienvenue chez CraftData, l'écosystème qui accélère l'innovation en Data et IA. Explorez nos services, ...",
    },
  ];

  if (searchInput && searchSuggestions) {
    searchInput.addEventListener("input", function () {
      const query = searchInput.value.toLowerCase();
      searchSuggestions.innerHTML = ""; // Clear previous suggestions

      if (query.length === 0) {
        searchSuggestions.style.display = "none";
        return;
      }

      const filteredSuggestions = searchData.filter(
        (item) =>
          item.keyword.toLowerCase().includes(query) ||
          item.content.toLowerCase().includes(query),
      );

      if (filteredSuggestions.length > 0) {
        filteredSuggestions.forEach((item) => {
          const suggestionItem = document.createElement("div");
          suggestionItem.classList.add("search-suggestion-item");
          suggestionItem.textContent = item.keyword; // Display keyword, but search on content
          suggestionItem.dataset.url = item.url;
          suggestionItem.addEventListener("click", function () {
            window.location.href = this.dataset.url;
          });
          searchSuggestions.appendChild(suggestionItem);
        });
        searchSuggestions.style.display = "block";
      } else {
        searchSuggestions.style.display = "none";
      }
    });

    // Hide suggestions when clicking outside
    document.addEventListener("click", function (event) {
      if (
        !searchInput.contains(event.target) &&
        !searchSuggestions.contains(event.target)
      ) {
        searchSuggestions.style.display = "none";
      }
    });
  }
});
