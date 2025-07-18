# CraftData - Data et IA pour les PME

Site web professionnel pour CraftData, spécialisée dans les solutions data et IA pour les PME du Centre-Val de Loire. Développé avec Rust, Leptos et WebAssembly, déployé sur GitHub Pages.

## 🚀 Technologies Utilisées

- **Rust** - Langage de programmation système
- **Leptos** - Framework web moderne pour Rust
- **WebAssembly (WASM)** - Compilation pour le navigateur
- **Trunk** - Outil de build et serveur de développement
- **GitHub Pages** - Hébergement statique
- **GitHub Actions** - CI/CD automatisé

## 📋 Prérequis

Assurez-vous d'avoir installé :

1. **Rust** (version stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Target WASM**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Trunk** (outil de build)
   ```bash
   cargo install trunk
   ```

## 🛠️ Développement Local

### Démarrer le serveur de développement

```bash
cd softia
trunk serve
```

Le site sera accessible sur `http://127.0.0.1:8080/softia/`

### Build de production

```bash
trunk build --release
```

Les fichiers de production seront générés dans le dossier `dist/`.

## 📁 Structure du Projet

```
softia/
├── src/
│   └── lib.rs          # Application principale Leptos (point d'entrée WASM)
├── .github/
│   └── workflows/
│       └── deploy.yml  # Configuration GitHub Actions
├── dist/               # Fichiers de production (généré)
├── index.html          # Template HTML principal
├── style.css           # Styles CSS
├── Trunk.toml          # Configuration Trunk
├── Cargo.toml          # Configuration Rust
└── README.md           # Ce fichier
```

## 🌐 Architecture du Site

Le site est une Single Page Application (SPA) avec les pages suivantes :

- **Accueil** (`/`) - Page d'accueil avec proposition de valeur, statistiques marché
- **À propos** (`/about`) - Mission, approche, fondateur, structure entreprise
- **Services** (`/services`) - Offres Exploration et Bâtisseur, services additionnels
- **Contact** (`/contact`) - Formulaire de contact détaillé, informations entreprise

### Composants Principaux

- `App` - Composant racine avec routeur
- `Header` - Navigation principale
- `Footer` - Pied de page
- Pages individuelles pour chaque route

## 🚀 Déploiement sur GitHub Pages

### Configuration Automatique

Le déploiement est automatisé via GitHub Actions. À chaque push sur la branche `main` :

1. Le code est compilé avec Trunk
2. Les fichiers sont déployés sur GitHub Pages
3. Le site est accessible sur `https://[votre-username].github.io/softia/`

### Configuration Manuelle

1. **Activer GitHub Pages** dans les paramètres du repository :
   - Settings → Pages
   - Source : "GitHub Actions"

2. **Pousser le code** sur la branche `main`

3. **Vérifier le déploiement** dans l'onglet "Actions"

## 🔧 Configuration

### Modifier l'URL de déploiement

Dans `Trunk.toml`, modifiez :
```toml
public_url = "/votre-nom-de-repo/"
```

### Personnalisation du contenu

- **Métadonnées** : Modifiez `index.html`
- **Styles** : Éditez `style.css`
- **Contenu** : Modifiez les composants dans `src/lib.rs`

## 🎨 Personnalisation des Styles

Le site utilise un design moderne avec :
- Palette de couleurs : dégradé bleu/violet (`#667eea` → `#764ba2`)
- Typography : Segoe UI, Tahoma, Geneva, Verdana
- Design responsive pour mobile et desktop
- Animations CSS subtiles
- Sections spécialisées : statistiques, grilles de services, formulaires avancés

### Variables CSS Principales

```css
/* Couleurs principales */
--primary-gradient: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
--text-color: #333;
--background-color: #fff;
--accent-color: #667eea;
```

## 📱 Responsive Design

Le site est optimisé pour :
- Desktop (> 768px)
- Tablet (768px - 480px)
- Mobile (< 480px)

## 🔍 SEO et Accessibilité

- Métadonnées complètes (Open Graph, Twitter Cards, keywords)
- Structure HTML sémantique
- Support des lecteurs d'écran
- Navigation au clavier
- Respect des préférences de mouvement réduit
- Optimisation pour les recherches locales (Centre-Val de Loire)
- Mots-clés ciblés : data science, IA, PME, solutions sur mesure

## 🚨 Dépannage

### Erreur de compilation WASM
```bash
# Réinstaller le target WASM
rustup target remove wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown
```

### Erreur Trunk non trouvé
```bash
# Réinstaller Trunk
cargo install trunk --force
```

### Erreur "found more than one target artifact"
Cette erreur se produit si vous avez à la fois un `main.rs` et un `lib.rs`. Pour WASM, gardez seulement `lib.rs`.

### Erreur features "nightly"
Si vous utilisez Rust stable, supprimez les features "nightly" des dépendances Leptos dans `Cargo.toml`.

### Erreur de déploiement GitHub Pages
1. Vérifiez que GitHub Pages est activé
2. Vérifiez les permissions dans Settings → Actions → General
3. Vérifiez les logs dans l'onglet Actions

## 💼 Contenu Entreprise

Le site présente fidèlement l'activité de CraftData :

### Offres Principales
- **Offre Exploration** : Audit + démonstrateur (à partir de 4 000 € HT, 3-6 semaines)
- **Offre Bâtisseur** : Solution complète sur mesure (à partir de 15 000 € HT, 3-6 mois)

### Services Additionnels
- Support technique (300 €/mois)
- Hébergement cloud souverain (150 €/mois)
- Formations et modules IA spécialisés

### Cible
- PME de 10 à 250 salariés
- Secteurs industriels et B2B
- Région Centre-Val de Loire
- Interlocuteurs : dirigeants, DAF, responsables digitaux

## 📝 TODO / Améliorations Futures

- [ ] Intégrer un système de contact fonctionnel (formulaire vers email)
- [ ] Ajouter des études de cas clients
- [ ] Intégrer un CMS headless pour le contenu
- [ ] Ajouter des témoignages clients
- [ ] Implémenter un blog technique
- [ ] Ajouter des tests automatisés
- [ ] Optimiser les performances WASM
- [ ] Ajouter un service worker pour le cache
- [ ] Intégrer Google Analytics/tracking

## 📞 Support

Pour toute question ou problème :
- Créez une issue sur GitHub
- Consultez la documentation Leptos : https://leptos.dev
- Documentation Trunk : https://trunkrs.dev

## 📄 Licence

Ce projet est sous licence MIT. Voir le fichier LICENSE pour plus de détails.