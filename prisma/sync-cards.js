const sheets = require("@googleapis/sheets");
const { PrismaClient } = require("@prisma/client");

const capitalizeFirstLetter = (str) => {
	console.log({
		str,
		toLower: str.toLowerCase().charAt(0),
	});
	return str.charAt(0).toUpperCase() + str.slice(1);
};
const prisma = new PrismaClient();

function getAuthClient() {
	const client = sheets.auth.fromAPIKey(process.env.GOOGLE_API_KEY);
	return client;
}

// TODO: Replace with your actual Google Sheet ID
const spreadsheetId = "1_hAIi99jgRz9MjClbebsOr7R1m0QmhbYmxKplwRMY-I";

async function readSheetAndInsertCards() {
	const sheetsApi = sheets.sheets({ version: "v4", auth: getAuthClient() });

	try {
		const response = await sheetsApi.spreadsheets.values.get({
			spreadsheetId: spreadsheetId,
			range: "Dusane!A1:J",
		});

		const rows = response.data.values;
		if (!rows || rows.length === 0) {
			console.log("No data found.");
			return;
		}

		for (let i = 2; i < rows.length; i++) {
			const row = rows[i];
			const [
				name,
				effectStr,
				manaCost,
				rarity,
				nation,
				type,
				damage,
				health,
				url,
				deckLimit,
			] = row;
			const cardData = {
				name,
				rarity: capitalizeFirstLetter(rarity.toLowerCase()),
				nation,
				imgUrl: url,
				manaCost: parseInt(manaCost),
				cardType: type,
				damage: damage ? parseInt(damage) : undefined,
				health: health ? parseInt(health) : undefined,
				deckLimit: parseInt(deckLimit),
			};
			console.log("rariry chaeck", cardData.rarity);
			const effects = parseEffects(effectStr);

			try {
				const createdCard = await prisma.card.create({
					data: cardData,
				});

				await createAndLinkEffects(createdCard, effects);
			} catch (err) {
				if (err.message.includes("Unique constraint failed on the fields")) {
					console.log(
						`Card ${name} already exists in the database. Skipping...`,
					);
				} else {
					console.error(`Error creating card: ${err}`);
				}
			}
		}
	} catch (err) {
		console.error("Error:", err);
	}
}

function parseEffects(effectsString) {
	if (!effectsString) {
		return [];
	}

	const effects = effectsString
		.split("\n")
		.filter(Boolean)
		.map((effect) => {
			const parts = effect.trim().split(",");
			const name = parts[0];
			const amount = Number.parseInt(parts[1], 10);
			const targetType = parts[2]
				? capitalizeFirstLetter(parts[2].trim().toLowerCase())
				: null;
			const damageType = parts[3]
				? capitalizeFirstLetter(parts[3].trim().toLowerCase())
				: null;

			return {
				name,
				amount,
				targetType: targetType ?? undefined,
				damageType: damageType ?? undefined,
			};
		});

	return effects;
}

const stringToCamelCaseEffect = (str) =>
	// biome-ignore lint/style/useTemplate: <explanation>
	str
		.trim()
		.toLowerCase()
		.replace(/[^a-zA-Z0-9]+(.)/g, (m, chr) => chr.toUpperCase()) + "Effect";

async function createAndLinkEffects(card, effects) {
	for (const effect of effects) {
		const { name, amount, targetType, damageType } = effect;
		const effectModelName = capitalizeFirstLetter(
			stringToCamelCaseEffect(name),
		);
		let effectCreateData = {};

		switch (effectModelName) {
			case "DamageEffect":
				effectCreateData = {
					[effectModelName]: {
						create: { damage: { create: { amount, targetType, damageType } } },
					},
				};
				break;
			case "HealEffect":
				effectCreateData = {
					[effectModelName]: {
						create: {
							amount,
							targetType,
						},
					},
				};
				break;
			case "LifestealEffect":
			case "PickupEffect":
			case "ArmorEffect":
			case "BattleCryEffect":
			case "CowardiceCurseEffect":
			case "PhantomTouchEffect":
			case "SprayOfKnivesEffect":
			case "CleanseEffect":
			case "RoarAuraEffect":
			case "DyingWishHealEffect":
			case "ResilienceEffect":
			case "PoisonEffect":
			case "InitiativeEffect":
				effectCreateData = { [effectModelName]: { create: { amount } } };
				break;
			default:
				effectCreateData = { [effectModelName]: {} };
				console.warn(
					`Effect type '${effectModelName}' not handled in the switch statement.`,
				);
				continue;
		}

		try {
			// Create CardEffect first
			const cardEffect = await prisma.cardEffect.create({
				data: {
					cardId: card.id,
				},
			});

			// Link the specific effect to CardEffect
			if (card.cardType === "Minion") {
				const minionEffect = await prisma.minionEffect.create({
					data: effectCreateData,
				});
				await prisma.cardEffect.update({
					where: { id: cardEffect.id },
					data: { minionEffectId: minionEffect.id },
				});
			} else {
				const spellEffect = await prisma.spellEffect.create({
					data: effectCreateData,
				});
				await prisma.cardEffect.update({
					where: { id: cardEffect.id },
					data: { spellEffectId: spellEffect.id },
				});
			}

			console.log(
				`Created ${card.cardType} effect for card ${card.name}:`,
				cardEffect,
			);
		} catch (err) {
			console.log(`Error creating card effect: ${err}`);
		}
	}
}

readSheetAndInsertCards();
