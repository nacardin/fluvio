#!/bin/bash

set -xe

ssh-keygen -q -N "" -t RSA -f id_rsa

aws ec2 import-key-pair --key-name fluvio-ci-$GITHUB_RUN_ID --public-key-material fileb://id_rsa.pub

ec2_instance_id=$(aws ec2 run-instances \
    --image-id $EC2_AMI \
    --instance-type c5.4xlarge \
    --security-group-ids sg-010f0a9ee7ddac50f \
    --key-name fluvio-ci-$GITHUB_RUN_ID \
    --iam-instance-profile SSM \
    --query "Instances[0].InstanceId" \
    --output text)

echo EC2_INSTANCE_ID=$ec2_instance_id >> $GITHUB_ENV

sleep 30

ec2_instance_public_ip=$(aws ec2 describe-instances \
    --instance-ids $ec2_instance_id \
    --query "Reservations[0].Instances[0].PublicIpAddress" \
    --output text)

echo EC2_INSTANCE_PUBLIC_IP=$ec2_instance_public_ip >> $GITHUB_ENV

ssh_opts="-o IdentitiesOnly=yes -o BatchMode=yes -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null"
ssh_remote="ubuntu@$ec2_instance_public_ip"
ssh_exec="ssh -i ./id_rsa $ssh_remote $ssh_opts"

echo SSH_EXEC=$ssh_exec >> $GITHUB_ENV
