import { PrismaClient, CardType, Class, EffectType, TargetType, StatType } from '@prisma/client';
const prisma = new PrismaClient();

async function main() {
  const cards = [
    {
      name: "Fireball",
      cardType: CardType.Attack,
      class: Class.Wizard,
      cost: 2,
      zeal: 1,
      tier: 1,
      imgUrl: "https://example.com/fireball.jpg",
      effects: [
        { effectType: EffectType.Damage, value: 15, target: TargetType.Opponent }
      ]
    },
    {
      name: "Shield Wall",
      cardType: CardType.Defensive,
      class: Class.Fighter,
      cost: 2,
      zeal: 1,
      tier: 1,
      imgUrl: "https://example.com/shield-wall.jpg",
      effects: [
        { effectType: EffectType.Armor, value: 10, target: TargetType.Itself }
      ]
    },
    {
      name: "Quick Shot",
      cardType: CardType.Attack,
      class: Class.Ranger,
      cost: 1,
      zeal: 1,
      tier: 1,
      imgUrl: "https://example.com/quick-shot.jpg",
      effects: [
        { effectType: EffectType.Damage, value: 8, target: TargetType.Opponent }
      ]
    },
    {
      name: "Arcane Intellect",
      cardType: CardType.Utility,
      class: Class.Wizard,
      cost: 1,
      zeal: 1,
      tier: 1,
      imgUrl: "https://example.com/arcane-intellect.jpg",
      effects: [
        { effectType: EffectType.ManaGain, value: 2, target: TargetType.Itself },
        { effectType: EffectType.Draw, value: 1, target: TargetType.Itself }
      ]
    },
    {
      name: "Strengthen",
      cardType: CardType.Utility,
      class: Class.Fighter,
      cost: 2,
      zeal: 1,
      tier: 1,
      imgUrl: "https://example.com/strengthen.jpg",
      effects: [
        { effectType: EffectType.BuffStat, value: 3, target: TargetType.Itself, statAffected: StatType.Strength, duration: 3 }
      ]
    },
    {
      name: "Poison Dart",
      cardType: CardType.Attack,
      class: Class.Ranger,
      cost: 1,
      zeal: 1,
      tier: 1,
      imgUrl: "https://example.com/poison-dart.jpg",
      effects: [
        { effectType: EffectType.Damage, value: 3, target: TargetType.Opponent },
        { effectType: EffectType.Poison, value: 2, target: TargetType.Opponent, duration: 3 }
      ]
    },
    {
      name: "Healing Potion",
      cardType: CardType.Utility,
      class: Class.Fighter,
      cost: 2,
      zeal: 1,
      tier: 1,
      imgUrl: "https://example.com/healing-potion.jpg",
      effects: [
        { effectType: EffectType.Heal, value: 15, target: TargetType.Itself }
      ]
    },
    {
      name: "Frost Nova",
      cardType: CardType.Attack,
      class: Class.Wizard,
      cost: 3,
      zeal: 2,
      tier: 2,
      imgUrl: "https://example.com/frost-nova.jpg",
      effects: [
        { effectType: EffectType.Damage, value: 8, target: TargetType.Opponent },
        { effectType: EffectType.DebuffStat, value: 2, target: TargetType.Opponent, statAffected: StatType.Dexterity, duration: 2 }
      ]
    },
    {
      name: "Precise Shot",
      cardType: CardType.Attack,
      class: Class.Ranger,
      cost: 2,
      zeal: 1,
      tier: 1,
      imgUrl: "https://example.com/precise-shot.jpg",
      effects: [
        { effectType: EffectType.Damage, value: 10, target: TargetType.Opponent },
        { effectType: EffectType.BuffDamage, value: 20, target: TargetType.Itself, duration: 1, percentageModifier: true }
      ]
    },
    {
      name: "Berserker Rage",
      cardType: CardType.Utility,
      class: Class.Fighter,
      cost: 3,
      zeal: 2,
      tier: 2,
      imgUrl: "https://example.com/berserker-rage.jpg",
      effects: [
        { effectType: EffectType.BuffDamage, value: 50, target: TargetType.Itself, duration: 3, percentageModifier: true },
        { effectType: EffectType.DebuffStat, value: 5, target: TargetType.Itself, statAffected: StatType.Armor, duration: 3 }
      ]
    },
    {
      name: "Mana Surge",
      cardType: CardType.Utility,
      class: Class.Wizard,
      cost: 0,
      zeal: 1,
      tier: 1,
      imgUrl: "https://example.com/mana-surge.jpg",
      effects: [
        { effectType: EffectType.ManaGain, value: 3, target: TargetType.Itself }
      ]
    },
    {
      name: "Evasive Maneuver",
      cardType: CardType.Defensive,
      class: Class.Ranger,
      cost: 1,
      zeal: 1,
      tier: 1,
      imgUrl: "https://example.com/evasive-maneuver.jpg",
      effects: [
        { effectType: EffectType.BuffStat, value: 5, target: TargetType.Itself, statAffected: StatType.Dexterity, duration: 2 }
      ]
    },
    {
      name: "Crushing Blow",
      cardType: CardType.Attack,
      class: Class.Fighter,
      cost: 3,
      zeal: 2,
      tier: 2,
      imgUrl: "https://example.com/crushing-blow.jpg",
      effects: [
        { effectType: EffectType.Damage, value: 18, target: TargetType.Opponent },
        { effectType: EffectType.DebuffStat, value: 3, target: TargetType.Opponent, statAffected: StatType.Armor, duration: 2 }
      ]
    },
    {
      name: "Arcane Explosion",
      cardType: CardType.Attack,
      class: Class.Wizard,
      cost: 4,
      zeal: 2,
      tier: 2,
      imgUrl: "https://example.com/arcane-explosion.jpg",
      effects: [
        { effectType: EffectType.Damage, value: 12, target: TargetType.Opponent },
        { effectType: EffectType.DebuffDamage, value: 30, target: TargetType.Opponent, duration: 2, percentageModifier: true }
      ]
    },
    {
      name: "Volley",
      cardType: CardType.Attack,
      class: Class.Ranger,
      cost: 3,
      zeal: 2,
      tier: 2,
      imgUrl: "https://example.com/volley.jpg",
      effects: [
        { effectType: EffectType.Damage, value: 5, target: TargetType.Opponent },
        { effectType: EffectType.Damage, value: 5, target: TargetType.Opponent },
        { effectType: EffectType.Damage, value: 5, target: TargetType.Opponent }
      ]
    },
    {
      name: "Fortify",
      cardType: CardType.Defensive,
      class: Class.Fighter,
      cost: 2,
      zeal: 1,
      tier: 1,
      imgUrl: "https://example.com/fortify.jpg",
      effects: [
        { effectType: EffectType.BuffStat, value: 8, target: TargetType.Itself, statAffected: StatType.Armor, duration: 3 }
      ]
    },
    {
      name: "Polymorph",
      cardType: CardType.Utility,
      class: Class.Wizard,
      cost: 4,
      zeal: 2,
      tier: 2,
      imgUrl: "https://example.com/polymorph.jpg",
      effects: [
        { effectType: EffectType.DebuffDamage, value: 70, target: TargetType.Opponent, duration: 2, percentageModifier: true },
        { effectType: EffectType.Silence, value: 1, target: TargetType.Opponent, duration: 2 }
      ]
    },
    {
      name: "Stealth",
      cardType: CardType.Utility,
      class: Class.Ranger,
      cost: 2,
      zeal: 1,
      tier: 1,
      imgUrl: "https://example.com/stealth.jpg",
      effects: [
        { effectType: EffectType.BuffStat, value: 10, target: TargetType.Itself, statAffected: StatType.Dexterity, duration: 2 },
        { effectType: EffectType.Draw, value: 1, target: TargetType.Itself }
      ]
    },
    {
      name: "Whirlwind",
      cardType: CardType.Attack,
      class: Class.Fighter,
      cost: 3,
      zeal: 2,
      tier: 2,
      imgUrl: "https://example.com/whirlwind.jpg",
      effects: [
        { effectType: EffectType.Damage, value: 12, target: TargetType.Opponent },
        { effectType: EffectType.BuffStat, value: 3, target: TargetType.Itself, statAffected: StatType.Armor, duration: 2 }
      ]
    },
    {
      name: "Time Warp",
      cardType: CardType.Utility,
      class: Class.Wizard,
      cost: 5,
      zeal: 3,
      tier: 3,
      imgUrl: "https://example.com/time-warp.jpg",
      effects: [
        { effectType: EffectType.ManaGain, value: 5, target: TargetType.Itself },
        { effectType: EffectType.Draw, value: 2, target: TargetType.Itself },
        { effectType: EffectType.BuffStat, value: 2, target: TargetType.Itself, statAffected: StatType.Intelligence, duration: 3 }
      ]
    }
  ];

  for (const cardData of cards) {
    try {
      const { effects, ...cardInfo } = cardData;
      const card = await prisma.card.create({
        data: {
          ...cardInfo,
          effects: {
            create: effects.map(effect => ({
              effectType: effect.effectType,
              value: effect.value,
              target: effect.target,
              duration: effect.duration,
              statAffected: effect.statAffected,
              percentageModifier: effect.percentageModifier || false
            }))
          }
        },
        include: {
          effects: true
        }
      });
      console.log(`Created card: ${card.name}`);
    } catch (error) {
      console.error(`Error creating card ${cardData.name}:`, error);
    }
  }

  console.log("Card insertion complete!");
}

main()
  .catch((e) => {
    console.error(e);
    process.exit(1);
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
