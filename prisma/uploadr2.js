const accountid = "0bd5ed1a4e347b9ac9b7360771c49626";
const accessKeyId = process.env.R2_ACCESS_KEY_ID;
const secretAccessKey = process.env.R2_SECRET_ACCESS_KEY;
const bucketName = "aion-rising-nft";

const devPubUrl = "https://pub-fe5f214823d746c1901f4d5c149ed48e.r2.dev/";

import {
	S3Client,
	ListBucketsCommand,
	ListObjectsV2Command,
	GetObjectCommand,
	PutObjectCommand,
} from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";

const S3 = new S3Client({
	region: "auto",
	endpoint: `https://${accountid}.r2.cloudflarestorage.com`,
	credentials: {
		accessKeyId: accessKeyId,
		secretAccessKey: secretAccessKey,
	},
});

const exampleFiles = [
	{
		name: "Fireball",
		type: "Relic",
		effect: {
			type: "Damage",
			value: 20,
		},
	},
	{
		name: "Strike",
		type: "Card",
		effect: {
			type: "Basic Attack",
			value: 6,
		},
	}
];

async function uploadJsonToR2(fileJson) {
	const jsonString = JSON.stringify(fileJson);

	const Key = fileJson.name;
	try {
		const params = {
			Bucket: bucketName,
			Key,
			Body: jsonString,
			ContentType: "application/json",
		};

		await S3.send(new PutObjectCommand(params));
		const pubUrl = `${devPubUrl}${Key}`;

		console.log("File uploaded successfully.", pubUrl);
	} catch (error) {
		console.error("Error uploading file:", error);
	}
}

for (const file of exampleFiles) {
	uploadJsonToR2(file);
}
