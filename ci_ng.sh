#!/bin/bash

set -xe

ssh-keygen -q -N "" -t RSA -f id_rsa

aws ec2 import-key-pair --key-name fluvio-ci-$GITHUB_RUN_ID --public-key-material fileb://id_rsa.pub || true

ec2_instance_id=$(aws ec2 run-instances \
    --image-id $EC2_AMI \
    --instance-type c5.4xlarge \
    --security-group-ids sg-010f0a9ee7ddac50f \
    --key-name fluvio-ci-$GITHUB_RUN_ID \
    --iam-instance-profile Arn=arn:aws:iam::808581242538:instance-profile/SSM \
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

tmp_dir=/tmp

$ssh_exec "export"
$ssh_exec "cat /home/ubuntu/.ssh/environment"

$ssh_exec 'echo RELEASE=true >> /home/ubuntu/.ssh/environment'

$ssh_exec "export"

echo SSH_EXEC_1="$ssh_exec cd $tmp_dir;" >> $GITHUB_ENV
echo SSH_EXEC="$ssh_exec export PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/home/ubuntu/.cargo/bin;export HOME=/home/ubuntu;cd $tmp_dir/fluvio;" >> $GITHUB_ENV

# export DEBIAN_FRONTEND=noninteractive
# sudo apt-get install git -y -qq
