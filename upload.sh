
# Set variables
CONTAINER_NAME="aion_server-postgres-1"
DB_USERNAME="your_db_username"
DB_NAME="defaultdb"
TIMESTAMP=$(date +%Y-%m-%dT%H-%M-%S)
BACKUP_FILE="/tmp/${DB_NAME}-${TIMESTAMP}.db"
LOCAL_BACKUP_FILE="${DB_NAME}-${TIMESTAMP}.db"
DROPBOX_CLIENT_ID="axb5phlqgsg02zr"
DROPBOX_CLIENT_SECRET="tgj215dkk8dghtc"
DROPBOX_PATH="/mbilaldbbackup"
REFRESH_TOKEN="OORz7B_uyRAAAAAAAAAAAQ0uOj4JoqArE2mpCipAhlfsQ_BirhCTmLJynlYncPY_"

get_new_access_token() {
  RESPONSE=$(curl -X POST https://api.dropboxapi.com/oauth2/token \
    -d grant_type=refresh_token \
    -d refresh_token=${REFRESH_TOKEN} \
    -d client_id=${DROPBOX_CLIENT_ID} \
    -d client_secret=${DROPBOX_CLIENT_SECRET})
  
  NEW_ACCESS_TOKEN=$(echo ${RESPONSE} | jq -r .access_token)
  
  echo $NEW_ACCESS_TOKEN
}

# run function in loop every minute
while true; do
    # Set variables
    CONTAINER_NAME="aion_server-postgres-1"
    DB_USERNAME="root"
    DB_NAME="defaultdb"
    TIMESTAMP=$(date +%Y-%m-%dT%H-%M-%S)
    BACKUP_FILE="/tmp/${DB_NAME}-${TIMESTAMP}.db"
    LOCAL_BACKUP_FILE="${DB_NAME}-${TIMESTAMP}.db"

    ACCESS_TOKEN=$(get_new_access_token)
    echo "Access token: ${ACCESS_TOKEN}"
    # Execute pg_dump inside the container
    docker exec ${CONTAINER_NAME} pg_dump -U ${DB_USERNAME} ${DB_NAME} -f ${BACKUP_FILE}
    echo "DB dump created."

    # Copy the backup file from the container to the local machine
    docker cp ${CONTAINER_NAME}:${BACKUP_FILE} .
    echo "Backup file copied to local machine."

    # Remove the backup file from the container
    docker exec ${CONTAINER_NAME} rm ${BACKUP_FILE}

    # Upload to google drive
    echo "Uploading ${LOCAL_BACKUP_FILE} to Dropbox..."

    curl -X POST https://content.dropboxapi.com/2/files/upload \
      --header "Authorization: Bearer ${ACCESS_TOKEN}" \
      --header "Dropbox-API-Arg: {\"path\": \"/${LOCAL_BACKUP_FILE}\",\"mode\": \"add\",\"autorename\": true,\"mute\": false}" \
      --header "Content-Type: application/octet-stream" \
      --data-binary @"${LOCAL_BACKUP_FILE}"


    echo "Upload complete."
    rm ${LOCAL_BACKUP_FILE}
    echo "Local backup file removed. Waiting 1 hour...."
    # Sleep for 60 seconds
    sleep 3600
done


