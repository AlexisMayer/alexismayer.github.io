
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
