#!/bin/bash

set -xe

ssh-keygen -q -t RSA -f id_rsa

aws ec2 import-key-pair --key-name fluvio-ci-$GITHUB_RUN_ID --public-key-material fileb://id_rsa.pub

ec2_instance_id=$(aws ec2 run-instances \
    --image-id $EC2_AMI \
    --instance-type m5.2xlarge \
    --security-group-ids sg-010f0a9ee7ddac50f \
    --key-name fluvio-ci-$GITHUB_RUN_ID \
    --query "Instances[0].InstanceId" \
    --output text)

echo ec2_instance_id
echo EC2_INSTANCE_ID=$ec2_instance_id >> $GITHUB_ENV

sleep 30

ec2_instance_public_ip=$(aws ec2 describe-instances \
    --instance-ids $ec2_instance_id \
    --query "Reservations[0].Instances[0].PublicIpAddress" \
    --output text)

echo ec2_instance_public_ip

ssh_opts=-o BatchMode=yes -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null
ssh_remote=ubuntu@$(cat ec2_instance_public_ip)
ssh_exec=ssh $SSH_REMOTE $SSH_OPTS

$ssh_exec 'echo test'

echo SSH_EXEC=$ssh_exec >> $GITHUB_ENV
