const { PrismaClient } = require("@prisma/client");

const prisma = new PrismaClient();
// enum Nation {
//     Dusane
// Aylen
// Ironmark
// Kelidor
// Meta
// }
const getRandomNation =()=> {
    const nations = ["Dusane", "Aylen", "Ironmark", "Kelidor", "Meta"];
    return nations[Math.floor(Math.random() * nations.length)];

}

async function createCardWithEffects(cardData, effects) {
    const { name, nation, rarity, manaCost, health, damage, cardType, tier, imgUrl } = cardData;
    const card = await prisma.card.create({
        data: {
            name,
            nation: getRandomNation(),
            rarity,
            manaCost,
            health,
            damage,
            cardType,
            tier,
            imgUrl,
        },
    });

    for (const effect of effects) {
        const { type, amount, targetType, damageType } = effect;
        let effectCreateData = {};
        switch (type) {
            case "PoisonEffect":
                effectCreateData = {
                    poisonEffect: {
                        create: { amount },
                    },
                };
                break;
            case "InitiativeEffect":
                effectCreateData = {
                    initiativeEffect: {
                        create: { amount },
                    },
                };
                break;
            case "DamageEffect":
                effectCreateData = {
                    damageEffect: {
                        create: { amount, damageType, targetType },
                    },
                };
                break;
            case "TauntEffect":
                effectCreateData = {
                    tauntEffect: {
                        create: {},
                    },
                };
                break;
            case "ChargeEffect":
                effectCreateData = {
                    chargeEffect: {
                        create: {},
                    },
                };
                break;
            case "LifestealEffect":
                effectCreateData = {
                    lifestealEffect: {
                        create: {},
                    },
                };
                break;
            case "PickupEffect":
                effectCreateData = {
                    pickupEffect: {
                        create: { amount },
                    },
                };
                break;
            case "SummonEffect":
                effectCreateData = {
                    summonEffect: {
                        create: {},
                    },
                };
                break;
            case "ResilienceEffect":
                effectCreateData = {
                    resilienceEffect: {
                        create: { amount },
                    },
                };
                break;
            // Add any additional effect types here
        }

        if (cardType === "Minion") {
            await prisma.minionEffect.create({
                data: {
                    card: { connect: { id: card.id } },
                    effects: { create: [effectCreateData] },
                },
            });
        } else if (cardType === "Spell") {
            await prisma.spellEffect.create({
                data: {
                    card: { connect: { id: card.id } },
                    effects: { create: [effectCreateData] },
                },
            });
        }
    }

    console.log(`${name} inserted successfully!`);
}


async function insertNewCards() {
// Sylvan Archer with ChargeEffect (Minion specific effect)
    await createCardWithEffects({
        name: "Sylvan Archer",
        nation: "Kelidor",
        rarity: "Common",
        manaCost: 2,
        health: 3,
        damage: 2,
        cardType: "Minion",
        tier: 1,
        imgUrl: "https://example.com/sylvanarcher.jpg",
    }, [{
        type: "ChargeEffect",
    }]);

// Frostfire Bolt (Spell effects only)
    await createCardWithEffects({
        name: "Frostfire Bolt",
        nation: "Dusane",
        rarity: "Magic",
        manaCost: 3,
        health: 0,
        damage: 0,
        cardType: "Spell",
        tier: 2,
        imgUrl: "https://example.com/frostfirebolt.jpg",
    }, [{
        type: "DamageEffect",
        amount: 3,
        targetType: "Hero",
        damageType: "Spell",
    }]);

// Ironclad Defender with TauntEffect (Minion specific effect)
    await createCardWithEffects({
        name: "Ironclad Defender",
        nation: "Ironmark",
        rarity: "Epic",
        manaCost: 4,
        health: 6,
        damage: 2,
        cardType: "Minion",
        tier: 3,
        imgUrl: "https://example.com/ironcladdefender.jpg",
    }, [{
        type: "TauntEffect",
    }]);

// Arcane Enlightenment (Spell does not have minion-specific effects like PickupEffect)
    await createCardWithEffects({
        name: "Arcane Enlightenment",
        nation: "Neuhaven",
        rarity: "Magic",
        manaCost: 2,
        health: 0,
        damage: 0,
        cardType: "Spell",
        tier: 2,
        imgUrl: "https://example.com/arcaneenlightenment.jpg",
    }, []);

// Blood Berserker with LifestealEffect (Minion specific effect)
    await createCardWithEffects({
        name: "Blood Berserker",
        nation: "Ironmark",
        rarity: "Legendary",
        manaCost: 6,
        health: 8,
        damage: 6,
        cardType: "Minion",
        tier: 3,
        imgUrl: "https://example.com/bloodberserker.jpg",
    }, [{
        type: "LifestealEffect",
    }]);

// Nature's Wrath (Spell effects only)
    await createCardWithEffects({
        name: "Nature's Wrath",
        nation: "Kelidor",
        rarity: "Epic",
        manaCost: 5,
        health: 0,
        damage: 0,
        cardType: "Spell",
        tier: 3,
        imgUrl: "https://example.com/natureswrath.jpg",
    }, [{
        type: "DamageEffect",
        amount: 4,
        targetType: "Minion",
        damageType: "Physical",
    }]);

// Divine Protector with LifestealEffect (Minion specific effect)
    await createCardWithEffects({
        name: "Divine Protector",
        nation: "Neuhaven",
        rarity: "Epic",
        manaCost: 7,
        health: 9,
        damage: 5,
        cardType: "Minion",
        tier: 3,
        imgUrl: "https://example.com/divineprotector.jpg",
    }, [{
        type: "LifestealEffect",
    }]);

// Shadowmeld Assassin (Minion specific effects like InitiativeEffect or StunEffect are not directly applicable to minions in the schema provided)
    await createCardWithEffects({
        name: "Shadowmeld Assassin",
        nation: "Kelidor",
        rarity: "Legendary",
        manaCost: 8,
        health: 5,
        damage: 8,
        cardType: "Minion",
        tier: 4,
        imgUrl: "https://example.com/shadowmeldassassin.jpg",
    }, []);

// Heaven's Fury with SpellEffects only
    await createCardWithEffects({
        name: "Heaven's Fury",
        nation: "Neuhaven",
        rarity: "Legendary",
        manaCost: 9,
        health: 0,
        damage: 0,
        cardType: "Spell",
        tier: 4,
        imgUrl: "https://example.com/heavensfury.jpg",
    }, [{
        type: "DamageEffect",
        amount: 7,
        targetType: "Both",
        damageType: "Spell",
    }, {
        type: "HealEffect",
        amount: 4,
        targetType: "Minion",
    }]);

// Goblin Sapper (Spell does not have minion-specific effects like SummonEffect or StunEffect)
    await createCardWithEffects({
        name: "Goblin Sapper",
        nation: "Meta",
        rarity: "Common",
        manaCost: 3,
        health: 2,
        damage: 4,
        cardType: "Minion",
        tier: 2,
        imgUrl: "https://example.com/goblinsapper.jpg",
    }, []);

// Mystic Barrier with SpellEffects only
    await createCardWithEffects({
        name: "Mystic Barrier",
        nation: "Neuhaven",
        rarity: "Magic",
        manaCost: 4,
        health: 0,
        damage: 0,
        cardType: "Spell",
        tier: 2,
        imgUrl: "https://example.com/mysticbarrier.jpg",
    }, [{
        type: "ResilienceEffect",
        amount: 3,
    }]);

// Ethereal Phantom with LifestealEffect (Minion specific effect)
    await createCardWithEffects({
        name: "Ethereal Phantom",
        nation: "Meta",
        rarity: "Epic",
        manaCost: 5,
        health: 4,
        damage: 6,
        cardType: "Minion",
        tier: 3,
        imgUrl: "https://example.com/etherealphantom.jpg",
    }, [{
        type: "LifestealEffect",
    }]);

// Thunderclap with SpellEffects only
    await createCardWithEffects({
        name: "Thunderclap",
        nation: "Dusane",
        rarity: "Magic",
        manaCost: 3,
        health: 0,
        damage: 0,
        cardType: "Spell",
        tier: 2,
        imgUrl: "https://example.com/thunderclap.jpg",
    }, [{
        type: "DamageEffect",
        amount: 2,
        targetType: "Minion",
        damageType: "Physical",
    }]);

// Cursed Witch (Minion specific effects)
    await createCardWithEffects({
        name: "Cursed Witch",
        nation: "Kelidor",
        rarity: "Legendary",
        manaCost: 6,
        health: 4,
        damage: 3,
        cardType: "Minion",
        tier: 3,
        imgUrl: "https://example.com/cursedwitch.jpg",
    }, [{
        type: "PoisonEffect",
        amount: 4,
    }]);

// Revitalize with SpellEffects only
    await createCardWithEffects({
        name: "Revitalize",
        nation: "Neuhaven",
        rarity: "Magic",
        manaCost: 2,
        health: 0,
        damage: 0,
        cardType: "Spell",
        tier: 2,
        imgUrl: "https://example.com/revitalize.jpg",
    }, [{
        type: "HealEffect",
        amount: 3,
        targetType: "Hero",
    }, {
        type: "ResilienceEffect",
        amount: 2,
    }]);

}

insertNewCards()
    .catch((error) => {
        console.error(error);
        process.exit(1);
    })
    .finally(async () => {
        await prisma.$disconnect();
    });
