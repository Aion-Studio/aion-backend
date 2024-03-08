const { PrismaClient } = require("@prisma/client");

const prisma = new PrismaClient();

async function insertMinions() {
  //enum Nation {
  //   Dusane
  //   Aylen
  //   Ironmark
  //   Kelidor
  //   Meta
  // }
  //
  //
  // enum Rarity {
  //   Common
  //   Magic
  //   Epic
  //   Legendary
  // }

  //
  //Lordaeron Footman
  // 1 Mana Cost
  // 2 Health, 1 Damage
  await prisma.card.create({
    data: {
      name: "Lordaeron Footman",
      nation: "Dusane",
      rarity: "Common",
      tier: 1,
      manaCost: 1,
      health: 2,
      damage: 1,
      cardType: "Minion",
    },
  });
  console.log("Lordaeron Footman inserted successfully!");
  //
  //Spellbreaker
  // 3 Mana Cost
  // 3 Health, 5 Damage
  // Effect: Upon being played, player can pickup +2 cards into their hand.

  const spellbreakerCard = await prisma.card.create({
    data: {
      name: "Spellbreaker",
      nation: "Aylen",
      rarity: "Common",
      tier: 2,
      manaCost: 3,
      health: 3,
      damage: 5,
      cardType: "Minion",
    },
  });
  const spellbreakerEffect = await prisma.pickupEffect.create({
    data: {
      amount: 2,
    },
  });
  await prisma.minionEffect.create({
    data: {
      card: {
        connect: { id: spellbreakerCard.id },
      },
      effects: {
        create: [
          {
            pickupEffect: {
              connect: { id: spellbreakerEffect.id },
            },
          },
        ],
      },
    },
  });

  console.log("Spellbreaker inserted successfully!");

  //Warg Rider
  // 5 Mana Cost
  // 5 Health, 7 Damage
  // Effect: Apply +2 Poison to enemy Hero when played

  const wargRiderCard = await prisma.card.create({
    data: {
      name: "Warg Rider",
      nation: "Ironmark",
      rarity: "Epic",
      tier: 3,
      manaCost: 5,
      health: 5,
      damage: 7,
      cardType: "Minion",
    },
  });
  const poisonEffect = await prisma.poisonEffect.create({
    data: {
      amount: 2,
    },
  });
  await prisma.minionEffect.create({
    data: {
      card: {
        connect: { id: wargRiderCard.id },
      },
      effects: {
        create: [
          {
            poisonEffect: {
              connect: { id: poisonEffect.id },
            },
          },
        ],
      },
    },
  });
  console.log("Warg Rider inserted successfully!");
}

async function insertSpells() {
  // Blizzard
  const blizzardCard = await prisma.card.create({
    data: {
      name: "Blizzard",
      nation: "Dusane",
      rarity: "Common",
      tier: 2,
      manaCost: 4,
      health: 0,
      damage: 0,
      cardType: "Spell",
    },
  });

  const blizzardMinionDamage = await prisma.damageEffect.create({
    data: {
      amount: 4,
      damageType: "Spell",
      targetType: "Minion",
    },
  });

  const blizzardHeroDamage = await prisma.damageEffect.create({
    data: {
      amount: 2,
      damageType: "Spell",
      targetType: "Hero",
    },
  });

  await prisma.spellEffect.create({
    data: {
      card: {
        connect: { id: blizzardCard.id },
      },
      effects: {
        create: [
          {
            damageEffect: {
              connect: { id: blizzardMinionDamage.id },
            },
          },
          {
            damageEffect: {
              connect: { id: blizzardHeroDamage.id },
            },
          },
        ],
      },
    },
  });

  console.log("Blizzard inserted successfully!");

  // Pulverize
  const pulverizeCard = await prisma.card.create({
    data: {
      name: "Pulverize",
      nation: "Ironmark",
      rarity: "Common",
      tier: 1,
      manaCost: 3,
      health: 0,
      damage: 0,
      cardType: "Spell",
    },
  });

  const pulverizeMinionDamage = await prisma.damageEffect.create({
    data: {
      amount: 2,
      damageType: "Physical",
      targetType: "Minion",
    },
  });

  const pulverizeHeroDamage = await prisma.damageEffect.create({
    data: {
      amount: 1,
      damageType: "Physical",
      targetType: "Hero",
    },
  });

  await prisma.spellEffect.create({
    data: {
      card: {
        connect: { id: pulverizeCard.id },
      },
      effects: {
        create: [
          {
            damageEffect: {
              connect: { id: pulverizeMinionDamage.id },
            },
          },
          {
            damageEffect: {
              connect: { id: pulverizeHeroDamage.id },
            },
          },
        ],
      },
    },
  });

  console.log("Pulverize inserted successfully!");

  // Healing Light
  const healingLightCard = await prisma.card.create({
    data: {
      name: "Healing Light",
      nation: "Aylen",
      rarity: "Common",
      tier: 2,
      manaCost: 5,
      health: 0,
      damage: 0,
      cardType: "Spell",
    },
  });

  const healingLightEffect = await prisma.healEffect.create({
    data: {
      amount: 5,
      targetType: "Both",
    },
  });

  await prisma.spellEffect.create({
    data: {
      card: {
        connect: { id: healingLightCard.id },
      },
      effects: {
        create: [
          {
            healEffect: {
              connect: { id: healingLightEffect.id },
            },
          },
        ],
      },
    },
  });

  console.log("Healing Light inserted successfully!");

  // Lesser Cleanse
  const lesserCleanseCard = await prisma.card.create({
    data: {
      name: "Lesser Cleanse",
      nation: "Aylen",
      rarity: "Common",
      tier: 1,
      manaCost: 1,
      health: 0,
      damage: 0,
      cardType: "Spell",
    },
  });

  const lesserCleanseEffect = await prisma.poisonEffect.create({
    data: {
      amount: -1,
    },
  });

  await prisma.spellEffect.create({
    data: {
      card: {
        connect: { id: lesserCleanseCard.id },
      },
      effects: {
        create: [
          {
            poisonEffect: {
              connect: { id: lesserCleanseEffect.id },
            },
          },
        ],
      },
    },
  });

  console.log("Lesser Cleanse inserted successfully!");

  // Cleanse
  const cleanseCard = await prisma.card.create({
    data: {
      name: "Cleanse",
      nation: "Aylen",
      rarity: "Common",
      tier: 2,
      manaCost: 4,
      health: 0,
      damage: 0,
      cardType: "Spell",
    },
  });

  const cleanseEffect = await prisma.poisonEffect.create({
    data: {
      amount: -5,
    },
  });

  await prisma.spellEffect.create({
    data: {
      card: {
        connect: { id: cleanseCard.id },
      },
      effects: {
        create: [
          {
            poisonEffect: {
              connect: { id: cleanseEffect.id },
            },
          },
        ],
      },
    },
  });

  console.log("Cleanse inserted successfully!");

  // Master Tactics
  const masterTacticsCard = await prisma.card.create({
    data: {
      name: "Master Tactics",
      nation: "Dusane",
      rarity: "Common",
      tier: 1,
      manaCost: 1,
      health: 0,
      damage: 0,
      cardType: "Spell",
    },
  });

  const masterTacticsEffect = await prisma.initiativeEffect.create({
    data: {
      amount: 1,
    },
  });

  await prisma.spellEffect.create({
    data: {
      card: {
        connect: { id: masterTacticsCard.id },
      },
      effects: {
        create: [
          {
            initiativeEffect: {
              connect: { id: masterTacticsEffect.id },
            },
          },
        ],
      },
    },
  });

  console.log("Master Tactics inserted successfully!");

  // Chaos Bolt
  const chaosBoltCard = await prisma.card.create({
    data: {
      name: "Chaos Bolt",
      nation: "Kelidor",
      rarity: "Epic",
      tier: 3,
      manaCost: 5,
      health: 0,
      damage: 0,
      cardType: "Spell",
    },
  });

  const chaosBoltEffect = await prisma.damageEffect.create({
    data: {
      amount: 6,
      damageType: "Chaos",
      targetType: "Hero",
    },
  });

  await prisma.spellEffect.create({
    data: {
      card: {
        connect: { id: chaosBoltCard.id },
      },
      effects: {
        create: [
          {
            damageEffect: {
              connect: { id: chaosBoltEffect.id },
            },
          },
        ],
      },
    },
  });

  console.log("Chaos Bolt inserted successfully!");

  // Death Pact
  const deathPactCard = await prisma.card.create({
    data: {
      name: "Death Pact",
      nation: "Kelidor",
      rarity: "Common",
      tier: 2,
      manaCost: 4,
      health: 0,
      damage: 0,
      cardType: "Spell",
    },
  });

  const deathPactDamage = await prisma.damageEffect.create({
    data: {
      amount: 4,
      damageType: "Spell",
      targetType: "Minion",
    },
  });

  const deathPactHeal = await prisma.healEffect.create({
    data: {
      amount: 2,
      targetType: "Hero",
    },
  });

  await prisma.spellEffect.create({
    data: {
      card: {
        connect: { id: deathPactCard.id },
      },
      effects: {
        create: [
          {
            damageEffect: {
              connect: { id: deathPactDamage.id },
            },
          },
          {
            healEffect: {
              connect: { id: deathPactHeal.id },
            },
          },
        ],
      },
    },
  });

  console.log("Death Pact inserted successfully!");

  // Shieldwall
  const shieldwallCard = await prisma.card.create({
    data: {
      name: "Shieldwall",
      nation: "Dusane",
      rarity: "Common",
      tier: 1,
      manaCost: 2,
      health: 0,
      damage: 0,
      cardType: "Spell",
    },
  });

  const shieldwallEffect = await prisma.armorEffect.create({
    data: {
      amount: 2,
    },
  });

  await prisma.spellEffect.create({
    data: {
      card: {
        connect: { id: shieldwallCard.id },
      },
      effects: {
        create: [
          {
            armorEffect: {
              connect: { id: shieldwallEffect.id },
            },
          },
        ],
      },
    },
  });

  console.log("Shieldwall inserted successfully!");

  // Decimate
  const decimateCard = await prisma.card.create({
    data: {
      name: "Decimate",
      nation: "Ironmark",
      rarity: "Epic",
      tier: 4,
      manaCost: 8,
      health: 0,
      damage: 0,
      cardType: "Spell",
    },
  });

  const decimateEffect = await prisma.armorEffect.create({
    data: {
      amount: -10,
    },
  });

  await prisma.spellEffect.create({
    data: {
      card: {
        connect: { id: decimateCard.id },
      },
      effects: {
        create: [
          {
            armorEffect: {
              connect: { id: decimateEffect.id },
            },
          },
        ],
      },
    },
  });

  console.log("Decimate inserted successfully!");

  // Shadowstrike
  const shadowstrikeCard = await prisma.card.create({
    data: {
      name: "Shadowstrike",
      nation: "Kelidor",
      rarity: "Epic",
      tier: 3,
      manaCost: 5,
      health: 0,
      damage: 0,
      cardType: "Spell",
    },
  });

  const shadowstrikeDamage = await prisma.damageEffect.create({
    data: {
      amount: 3,
      damageType: "Spell",
      targetType: "Both",
    },
  });

  const shadowstrikePoison = await prisma.poisonEffect.create({
    data: {
      amount: 2,
    },
  });

  await prisma.spellEffect.create({
    data: {
      card: {
        connect: { id: shadowstrikeCard.id },
      },
      effects: {
        create: [
          {
            damageEffect: {
              connect: { id: shadowstrikeDamage.id },
            },
          },
          {
            poisonEffect: {
              connect: { id: shadowstrikePoison.id },
            },
          },
        ],
      },
    },
  });

  console.log("Shadowstrike inserted successfully!");
}

insertSpells()
  .catch((error) => {
    console.error(error);
    process.exit(1);
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
