#!/bin/bash
# --feature-flag feature:feature-1, type: deployment, language: bash, environment: production

set -e

FEATURE_NAME="feature-1"
DEPLOY_DIR="/var/www/features"
BACKUP_DIR="/var/backups/features"

echo "Starting deployment of $FEATURE_NAME..."

# Create backup directory if it doesn't exist
if [ ! -d "$BACKUP_DIR" ]; then
    mkdir -p "$BACKUP_DIR"
    echo "Created backup directory: $BACKUP_DIR"
fi

# Backup existing deployment
if [ -d "$DEPLOY_DIR/$FEATURE_NAME" ]; then
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    echo "Backing up existing deployment..."
    cp -r "$DEPLOY_DIR/$FEATURE_NAME" "$BACKUP_DIR/${FEATURE_NAME}_${TIMESTAMP}"
    echo "Backup created: $BACKUP_DIR/${FEATURE_NAME}_${TIMESTAMP}"
fi

# Deploy new version
echo "Deploying new version..."
# Add your deployment commands here

echo "Deployment of $FEATURE_NAME completed successfully!"
