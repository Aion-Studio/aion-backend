const { PrismaClient } = require("@prisma/client");

const prisma = new PrismaClient();

async function createDeckForHero(heroId, cardIds) {
    try {
        // Create a new deck for the hero
        const deck = await prisma.deck.create({
            data: {
                Hero: {
                    connect: { id: heroId },
                },
            },
        });

        // Add the specified cards to the hero
        await Promise.all(
            cardIds.map((cardId) =>
                prisma.heroCard.create({
                    data: {
                        heroId,
                        cardId,
                    },
                })
            )
        );

        // Get the hero's cards
        const heroCards = await prisma.heroCard.findMany({
            where: {
                heroId,
            },
        });

        // Randomly select 8 cards from the hero's cards
        const selectedCards = [];
        for (let i = 0; i < 8; i++) {
            if (heroCards.length > 0) {
                const randomIndex = Math.floor(Math.random() * heroCards.length);
                const selectedCard = heroCards[randomIndex];
                selectedCards.push(selectedCard);
                heroCards.splice(randomIndex, 1);
            }
        }

        // Add the selected cards to the hero's deck
        await Promise.all(
            selectedCards.map((card) =>
                prisma.deckCard.create({
                    data: {
                        deckId: deck.id,
                        cardId: card.cardId,
                    },
                })
            )
        );

        console.log(`Deck created for hero ${heroId} with 8 random cards.`);
    } catch (error) {
        console.error(`Error creating deck for hero ${heroId}:`, error);
    }
}

async function createDeckForNpc(npcId, cardIds){
    try {
        // Create a new deck for the hero
        const deck = await prisma.deck.create({
            data: {
                Npc: {
                    connect: { id: npcId },
                },
            },
        });

        // Add the specified cards to the hero
        await Promise.all(
            cardIds.map((cardId) =>
                prisma.npcCard.create({
                    data: {
                        npcId,
                        cardId,
                    },
                })
            )
        );

        // Get the hero's cards
        const npcCards = await prisma.npcCard.findMany({
            where: {
                npcId,
            },
        });

        // Randomly select 8 cards from the hero's cards
        const selectedCards = [];
        for (let i = 0; i < 8; i++) {
            if (npcCards.length > 0) {
                const randomIndex = Math.floor(Math.random() * npcCards.length);
                const selectedCard = npcCards[randomIndex];
                selectedCards.push(selectedCard);
                npcCards.splice(randomIndex, 1);
            }
        }

        // Add the selected cards to the hero's deck
        await Promise.all(
            selectedCards.map((card) =>
                prisma.deckCard.create({
                    data: {
                        deckId: deck.id,
                        cardId: card.cardId,
                    },
                })
            )
        );

        console.log(`Deck created for npc ${npcId} with 8 random cards.`);
    } catch (error) {
        console.error(`Error creating deck for npc ${npcId}:`, error);
    }

}

const heroId= 'e75aba73-f132-4d1f-bab2-f16fd3518f86';
const cardIds = [
    'ab6243ee-b9cc-408a-9d6c-a0acf87a184d', 'b819a7b6-efe0-43ea-8a35-f01c4eb2438c', 'aa0c8a92-636f-413a-a88a-91fe5f10fa5c', 'a226626e-3737-4044-a5b5-29d3cd2f6c2d', '9abd0ddf-2b4d-4b42-9663-ca4447fe31b5', '854d1b68-ea54-4020-bcc3-a871ca422003', 'a400daa6-1079-4781-b3c4-10c12b94f43e', '071053a5-dc47-4a9a-88a3-ba8b8f3b4467'
]
// createDeckForHero(heroId,cardIds).catch((e) => {
//     console.error(error);
//     process.exit(1);
// }).finally(async () => {
//     await prisma.$disconnect();
// })
let npcCards = ['903f04a9-5bf8-474a-9fe4-02321540070c', 'b46ed5f9-2539-482d-a7e9-67e1e3f277a8', '86118326-4107-4ffe-95ae-9f3b21635980', 'dfd5f97e-c311-4c41-869a-dfc60c436943', '042421ac-4281-402c-817b-12c140a81c63', 'ab6243ee-b9cc-408a-9d6c-a0acf87a184d', 'f6267068-cf71-44c2-ba2d-97d46ffa21c6', '1f686337-a1da-4610-a212-39f372c4cbfc']
createDeckForNpc("ef5666e9-95ee-4097-a643-afef984efee5",npcCards).catch((e) => {
    console.error(error);
    process.exit(1);
}).finally(async () => {
    await prisma.$disconnect();
})