

## Instead of cloning the repo to the aws cloud 9, i pulled my docker image and pushed it to the ECR registry as below
### 1. pull the docker image
ec2-user:~/environment $ docker pull nehabardeduke/mockingjayactix:latest
latest: Pulling from nehabardeduke/mockingjayactix
29cd48154c03: Pull complete 
4f4fb700ef54: Pull complete 
ffbd143d7c9b: Pull complete 

Status: Downloaded newer image for nehabardeduke/mockingjayactix:latest
docker.io/nehabardeduke/mockingjayactix:latest
### 2. Login to the AWS ECR (the login credentials are changed from the originals to avoid misuse)
ec2-user:~/environment $ aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 5994147.dkr.ecr.us-east-1.amazonaws.com
WARNING! Your password will be stored unencrypted in /home/ec2-user/.docker/config.json.
Configure a credential helper to remove this warning. See
https://docs.docker.com/engine/reference/commandline/login/#credentials-store

Login Succeeded
ec2-user:~/environment $ docker images
REPOSITORY                      TAG       IMAGE ID       CREATED       SIZE
nehabardeduke/mockingjayactix   latest    6f6e9501adcc   2 hours ago   94.8MB

### 3. tag the docker image you pulled to the required ECR tag
ec2-user:~/environment $ docker tag nehabardeduke/mockingjayactix:latest 5994147.dkr.ecr.us-east-1.amazonaws.com/mockingjayactix:latest

### 4. Push to ECR registry
ec2-user:~/environment $ docker push 5994147.dkr.ecr.us-east-1.amazonaws.com/mockingjayactix:latest
The push refers to repository [5994147.dkr.ecr.us-east-1.amazonaws.com/mockingjayactix]
e37d70baed24: Pushed 
5f70bf18a086: Pushed 
63b3cf45ece8: Pushed 

ec2-user:~/environment $ 
