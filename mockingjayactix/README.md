Docker Image:
docker pull nehabardeduke/mockingjayactix:latest
'''bash
Instead of cloning the repo to the aws cloud 9, i simply pulled my docker image and pushed it to the ECR registry as below
ec2-user:~/environment $ docker pull nehabardeduke/mockingjayactix:latest
latest: Pulling from nehabardeduke/mockingjayactix
29cd48154c03: Pull complete 
4f4fb700ef54: Pull complete 
ffbd143d7c9b: Pull complete 
Digest: sha256:68dcd1078374b80124d1098a460658ca60f85ada5da445c096709b7947bca268
Status: Downloaded newer image for nehabardeduke/mockingjayactix:latest
docker.io/nehabardeduke/mockingjayactix:latest
ec2-user:~/environment $ aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 599412038047.dkr.ecr.us-east-1.amazonaws.com
WARNING! Your password will be stored unencrypted in /home/ec2-user/.docker/config.json.
Configure a credential helper to remove this warning. See
https://docs.docker.com/engine/reference/commandline/login/#credentials-store

Login Succeeded
ec2-user:~/environment $ docker images
REPOSITORY                      TAG       IMAGE ID       CREATED       SIZE
nehabardeduke/mockingjayactix   latest    6f6e9501adcc   2 hours ago   94.8MB
ec2-user:~/environment $ docker tag nehabardeduke/mockingjayactix:latest 599412038047.dkr.ecr.us-east-1.amazonaws.com/mockingjayactix:latest
ec2-user:~/environment $ docker push 599412038047.dkr.ecr.us-east-1.amazonaws.com/mockingjayactix:latest
The push refers to repository [599412038047.dkr.ecr.us-east-1.amazonaws.com/mockingjayactix]
e37d70baed24: Pushed 
5f70bf18a086: Pushed 
63b3cf45ece8: Pushed 
latest: digest: sha256:68dcd1078374b80124d1098a460658ca60f85ada5da445c096709b7947bca268 size: 946
ec2-user:~/environment $ 
