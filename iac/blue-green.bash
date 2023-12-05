# vars
green_dep_file=challenge.stack.yaml
blue_dep_file=challenge-blue.stack.yaml
server_name=ancgames.com/challenge
public_endpoint=https://ancgames.com/challenge
nginx_conf_file="/etc/nginx/nginx.conf"

# get random port
# Picks a random number between 3000 and 3999.
function random-number {
  floor=3000
  range=3999
  number=0
  while [ "$number" -le $floor ]
  do
    number=$RANDOM
    let "number %= $range"
  done
  echo $number
}

echo "Getting a new random port for the green switch"
new_port=$(random-number)
while [[ $(lsof -i -P -n | grep :$new_port) ]]
do
  new_port=$(random-number)
done

# now that we have our port lets sub in that port to the stack
echo "Setting new env variables"
sed -i.bak "s/\$PORT/$new_port/g" $green_dep_file

# okay now we should be able to spin up our green env
echo "Running green switch"
docker compose -f $green_dep_file up -d
green_logs=$(docker compose -f $green_dep_file logs -t)

echo "Checking green deployment"
if [[ $green_logs == *"panic: 1"* ]]; then
    echo "Green deployment failed."
    echo $green_logs
    echo "Spinning down green deployment"
    docker compose -f $green_dep_file down
    exit 1
fi

# need to wait for the docker container to spin up
sleep 7

# modify the nginx conf
echo "Updating nginx config using $new_port"
sed -i.bak "/server_name $server_name;/,/proxy_pass/s#proxy_pass http://localhost:[0-9]\+#proxy_pass http://localhost:$new_port#" $nginx_conf_file

echo "Restarting NGINX"
sudo systemctl restart nginx

# check the endpoint
title=$(curl -s $public_endpoint | grep -o '<title>.*</title>' | awk -F '<title>' '{print $2}' | awk -F '</title>' '{print $1}')
echo $title

echo "Checking public endpoint"
if [ "$title" != "Challenge" ]; then
    echo "Website is not being server resetting back."
    rm $nginx_conf_file
    cp $nginx_conf_file.bak $nginx_conf_file

    sudo systemctl restart nginx
    exit 1
fi

echo "Decommisioning blue switch"
docker compose -f $blue_dep_file down