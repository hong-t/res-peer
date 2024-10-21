#!/bin/bash

function cleanup() {
  kill -15 `ps | grep linera-proxy | awk '{print $1}'` > /dev/null 2>&1
  kill -15 `ps | grep linera-server | awk '{print $1}'` > /dev/null 2>&1
  kill -15 `ps | grep linera | awk '{print $1}'` > /dev/null 2>&1
  kill -15 `ps | grep socat | awk '{print $1}'` > /dev/null 2>&1
}

cleanup

BLUE='\033[1;34m'
YELLOW='\033[1;33m'
LIGHTGREEN='\033[1;32m'
NC='\033[0m'

unset RUSTFLAGS
unset TMPDIR

function print() {
  echo -e $1$2$3$NC
}

faucet_url=https://faucet.devnet-2024-09-04.linera.net

options="f:"
while getopts $options opt; do
  case ${opt} in
    f) faucet_url=${OPTARG} ;;
  esac
done

PROJECT_ROOT=$HOME/linera-project

NODE_LOG_FILE=$PROJECT_ROOT/linera.log
SERVICE_LOG_FILE=$PROJECT_ROOT/service_8080.log
WALLET_NUMBER=4
EXTRA_WALLET_NUMBER=`expr $WALLET_NUMBER - 1`

WALLET_BASE=$PROJECT_ROOT/linera/respeer
mkdir -p $WALLET_BASE
rm -rf $WALLET_BASE/*

print $'\U01F4AB' $YELLOW " Running linera net, log in $NODE_LOG_FILE ..."

function create_wallet() {
  export LINERA_WALLET_$1=$WALLET_BASE/wallet_$1.json
  export LINERA_STORAGE_$1=rocksdb:$WALLET_BASE/client_$1.db

  linera -w $1 wallet init --faucet $faucet_url --with-new-chain
  linera -w $1 wallet show
}

for i in `seq 0 $EXTRA_WALLET_NUMBER`; do
  create_wallet $i
done

function __run_service() {
  linera -w $1 service --port $2 --external-signing false
  if [ ! $? -eq 0 ]; then
    echo "Run with official release"
    linera -w $1 service --port $2
  fi
}

function run_service () {
  local_port=`expr 9080 + $1`
  pub_port=`expr 10100 + $1`

  __run_service $1 $local_port > $PROJECT_ROOT/service_$local_port.log 2>&1 &

  sleep 3
  socat TCP4-LISTEN:$pub_port TCP4:localhost:$local_port
}

print $'\U01F4AB' $YELLOW " Deploying Credit bytecode ..."
credit_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/credit_{contract,service}.wasm`
print $'\U01F4AB' $YELLOW " Creating Credit application ..."
credit_appid=`linera --with-wallet 0 create-application $credit_bid --json-argument '{"initial_supply":"99999999999999.0","amount_alive_ms":600000}'`
print $'\U01f499' $LIGHTGREEN " Credit application deployed"
echo -e "    Bytecode ID:    $BLUE$credit_bid$NC"
echo -e "    Application ID: $BLUE$credit_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Foundation application ..."
foundation_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/foundation_{contract,service}.wasm`
foundation_appid=`linera --with-wallet 0 create-application $foundation_bid --json-argument '{"review_reward_percent":20,"review_reward_factor":20,"author_reward_percent":40,"author_reward_factor":20,"activity_reward_percent":10}'`
print $'\U01f499' $LIGHTGREEN " Foundation application deployed"
echo -e "    Bytecode ID:    $BLUE$foundation_bid$NC"
echo -e "    Application ID: $BLUE$foundation_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Feed application ..."
feed_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/feed_{contract,service}.wasm`
feed_appid=`linera --with-wallet 0 create-application $feed_bid --json-argument '{"react_interval_ms":60000}' --json-parameters "{\"credit_app_id\":\"$credit_appid\",\"foundation_app_id\":\"$foundation_appid\"}" --required-application-ids $credit_appid --required-application-ids $foundation_appid`
print $'\U01f499' $LIGHTGREEN " Feed application deployed"
echo -e "    Bytecode ID:    $BLUE$feed_bid$NC"
echo -e "    Application ID: $BLUE$feed_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Market application ..."
market_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/market_{contract,service}.wasm`
market_appid=`linera --with-wallet 0 create-application $market_bid --json-argument '{"credits_per_linera":"30","max_credits_percent":30,"trade_fee_percent":3}' --json-parameters "{\"credit_app_id\":\"$credit_appid\",\"foundation_app_id\":\"$foundation_appid\"}" --required-application-ids $credit_appid --required-application-ids $foundation_appid`
print $'\U01f499' $LIGHTGREEN " Market application deployed"
echo -e "    Bytecode ID:    $BLUE$market_bid$NC"
echo -e "    Application ID: $BLUE$market_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Review application ..."
review_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/review_{contract,service}.wasm`
review_appid=`linera --with-wallet 0 create-application $review_bid --json-argument '{"content_approved_threshold":3,"content_rejected_threshold":2,"asset_approved_threshold":2,"asset_rejected_threshold":2,"reviewer_approved_threshold":2,"reviewer_rejected_threshold":2,"activity_approved_threshold":2,"activity_rejected_threshold":2}' --json-parameters "{\"feed_app_id\":\"$feed_appid\",\"credit_app_id\":\"$credit_appid\",\"foundation_app_id\":\"$foundation_appid\",\"market_app_id\":\"$market_appid\"}" --required-application-ids $feed_appid --required-application-ids $credit_appid --required-application-ids $foundation_appid --required-application-ids $market_appid`
print $'\U01f499' $LIGHTGREEN " Review application deployed"
echo -e "    Bytecode ID:    $BLUE$review_bid$NC"
echo -e "    Application ID: $BLUE$review_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Activity application ..."
activity_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/activity_{contract,service}.wasm`
activity_appid=`linera --with-wallet 0 create-application $activity_bid --json-parameters "{\"review_app_id\":\"$review_appid\",\"foundation_app_id\":\"$foundation_appid\",\"feed_app_id\":\"$feed_appid\"}" --required-application-ids $review_appid --required-application-ids $foundation_appid --required-application-ids $feed_appid`
print $'\U01f499' $LIGHTGREEN " Activity application deployed"
echo -e "    Bytecode ID:    $BLUE$activity_bid$NC"
echo -e "    Application ID: $BLUE$activity_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying BlobGateway application ..."
blob_gateway_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/blob_gateway_{contract,service}.wasm`
blob_gateway_appid=`linera --with-wallet 0 create-application $blob_gateway_bid`
print $'\U01f499' $LIGHTGREEN " BlobGateway application deployed"
echo -e "    Bytecode ID:    $BLUE$blob_gateway_bid$NC"
echo -e "    Application ID: $BLUE$blob_gateway_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying CPRegistry application ..."
cp_registry_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/cp_registry_{contract,service}.wasm`
cp_registry_appid=`linera --with-wallet 0 create-application $cp_registry_bid`
print $'\U01f499' $LIGHTGREEN " CPRegistry application deployed"
echo -e "    Bytecode ID:    $BLUE$cp_registry_bid$NC"
echo -e "    Application ID: $BLUE$cp_registry_appid$NC"

app_deploy_chain=`linera --with-wallet 0 wallet show | grep "Public Key" | awk '{print $2}'`
app_deploy_owner=`linera --with-wallet 0 wallet show | grep "Owner" | awk '{print $4}'`

print $'\U01F4AB' $YELLOW " Deploying Copilot CPU application ..."
copilot_cpu_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/copilot_{contract,service}.wasm`
copilot_cpu_appid=`linera --with-wallet 0 create-application $copilot_cpu_bid --json-argument "{\"node_id\":\"d7a776b018fefbd45d533d3031c101bb64c29d52423beb6e4d5cf84e322ef429\",\"brand_logo\":\"https://github.com/respeer-ai/res-peer/blob/master/webui/public/favicon.png?raw=true\",\"brand_name\":\"respeer.ai\",\"link_base\":\"http://172.16.31.73:9081\",\"resource_type\":\"CPU\",\"device_model\":\"Intel(R) Xeon(R) Silver 4214R CPU @ 2.40GHz\",\"cpu_model\":\"Intel(R) Xeon(R) Silver 4214R CPU @ 2.40GHz\",\"storage_type\":\"NVME\",\"storage_bytes\":100000000000,\"memory_bytes\":256000000000,\"free_quota\":3,\"price_quota\":1,\"quota_price\":\"0.003\",\"supported_task_types\":[\"FixGrammar\",\"RewriteEasierUnderstand\",\"Paraphrase\",\"WriteFormally\",\"WriteMoreNeutral\"],\"payment_chain_id\":\"$app_deploy_chain\",\"ai_model\":\"CoEDiT T5\",\"ai_model_url\":\"https://huggingface.co/jbochi/candle-coedit-quantized\"}" --json-parameters "{\"cp_registry_app_id\":\"$cp_registry_appid\"}" --required-application-ids $cp_registry_appid`
print $'\U01f499' $LIGHTGREEN " Copilot CPU application deployed"
echo -e "    Bytecode ID:    $BLUE$copilot_cpu_bid$NC"
echo -e "    Application ID: $BLUE$copilot_cpu_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Copilot GPU application ..."
copilot_fetch_server_url="http://localhost:9071/?prompt="
copilot_gpu_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/copilot_{contract,service}.wasm`
copilot_gpu_appid=`linera --with-wallet 0 create-application $copilot_gpu_bid --json-argument "{\"node_id\":\"d7a776b018fefbd45d533d3031c101bb64c29d52423beb6e4d5cf84e322ef429\",\"brand_logo\":\"https://github.com/respeer-ai/res-peer/blob/master/webui/public/favicon.png?raw=true\",\"brand_name\":\"respeer.ai\",\"link_base\":\"http://172.16.31.73:9081\",\"resource_type\":\"GPU\",\"device_model\":\"NVIDIA GeForce RTX 3090\",\"cpu_model\":\"Intel(R) Xeon(R) Silver 4214R CPU @ 2.40GHz\",\"storage_type\":\"NVME\",\"storage_bytes\":100000000000,\"memory_bytes\":256000000000,\"free_quota\":3,\"price_quota\":1,\"quota_price\":\"0.003\",\"supported_task_types\":[\"FixGrammar\",\"RewriteEasierUnderstand\",\"Paraphrase\",\"WriteFormally\",\"WriteMoreNeutral\"],\"payment_chain_id\":\"$app_deploy_chain\",\"fetch_server_url\":\"$copilot_fetch_server_url\",\"ai_model\":\"CoEDiT T5\",\"ai_model_url\":\"https://huggingface.co/jbochi/candle-coedit-quantized\"}" --json-parameters "{\"cp_registry_app_id\":\"$cp_registry_appid\"}" --required-application-ids $cp_registry_appid`
print $'\U01f499' $LIGHTGREEN " Copilot GPU application deployed"
echo -e "    Bytecode ID:    $BLUE$copilot_gpu_bid$NC"
echo -e "    Application ID: $BLUE$copilot_gpu_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Illustrator CPU application ..."
illustrator_cpu_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/illustrator_{contract,service}.wasm`
illustrator_cpu_appid=`linera --with-wallet 0 create-application $illustrator_cpu_bid --json-argument "{\"node_id\":\"d7a776b018fefbd45d533d3031c101bb64c29d52423beb6e4d5cf84e322ef429\",\"brand_logo\":\"https://github.com/respeer-ai/res-peer/blob/master/webui/public/favicon.png?raw=true\",\"brand_name\":\"respeer.ai\",\"link_base\":\"http://172.16.31.73:9081\",\"resource_type\":\"CPU\",\"device_model\":\"Intel(R) Xeon(R) Silver 4214R CPU @ 2.40GHz\",\"cpu_model\":\"Intel(R) Xeon(R) Silver 4214R CPU @ 2.40GHz\",\"storage_type\":\"NVME\",\"storage_bytes\":100000000000,\"memory_bytes\":256000000000,\"free_quota\":3,\"price_quota\":1,\"quota_price\":\"0.03\",\"supported_task_types\":[\"GenerateIllustrate\"],\"payment_chain_id\":\"$app_deploy_chain\",\"ai_model\":\"Tiny Stable Diffusion\",\"ai_model_url\":\"https://huggingface.co/segmind/tiny-sd\"}" --json-parameters "{\"cp_registry_app_id\":\"$cp_registry_appid\"}" --required-application-ids $cp_registry_appid`
print $'\U01f499' $LIGHTGREEN " Illustrator CPU application deployed"
echo -e "    Bytecode ID:    $BLUE$illustrator_cpu_bid$NC"
echo -e "    Application ID: $BLUE$illustrator_cpu_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Illustrator GPU application ..."
illustrator_fetch_server_url="http://localhost:9072/?prompt="
illustrator_gpu_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/illustrator_{contract,service}.wasm`
illustrator_gpu_appid=`linera --with-wallet 0 create-application $illustrator_gpu_bid --json-argument "{\"node_id\":\"d7a776b018fefbd45d533d3031c101bb64c29d52423beb6e4d5cf84e322ef429\",\"brand_logo\":\"https://github.com/respeer-ai/res-peer/blob/master/webui/public/favicon.png?raw=true\",\"brand_name\":\"respeer.ai\",\"link_base\":\"http://172.16.31.73:9081\",\"resource_type\":\"GPU\",\"device_model\":\"NVIDIA GeForce RTX 3090\",\"cpu_model\":\"Intel(R) Xeon(R) Silver 4214R CPU @ 2.40GHz\",\"storage_type\":\"NVME\",\"storage_bytes\":100000000000,\"memory_bytes\":256000000000,\"free_quota\":3,\"price_quota\":1,\"quota_price\":\"0.03\",\"supported_task_types\":[\"GenerateIllustrate\"],\"payment_chain_id\":\"$app_deploy_chain\",\"fetch_server_url\":\"$illustrator_fetch_server_url\",\"ai_model\":\"Tiny Stable Diffusion\",\"ai_model_url\":\"https://huggingface.co/segmind/tiny-sd\"}" --json-parameters "{\"cp_registry_app_id\":\"$cp_registry_appid\"}" --required-application-ids $cp_registry_appid`
print $'\U01f499' $LIGHTGREEN " Illustrator GPU application deployed"
echo -e "    Bytecode ID:    $BLUE$illustrator_gpu_bid$NC"
echo -e "    Application ID: $BLUE$illustrator_gpu_appid$NC"

sed -i "s/feedApp =.*/feedApp = '$feed_appid',/g" webui/src/const/index.ts
sed -i "s/creditApp =.*/creditApp = '$credit_appid',/g" webui/src/const/index.ts
sed -i "s/marketApp =.*/marketApp = '$market_appid',/g" webui/src/const/index.ts
sed -i "s/reviewApp =.*/reviewApp = '$review_appid',/g" webui/src/const/index.ts
sed -i "s/foundationApp =.*/foundationApp = '$foundation_appid'/g," webui/src/const/index.ts
sed -i "s/activityApp =.*/activityApp = '$activity_appid',/g" webui/src/const/index.ts
sed -i "s/blobGatewayApp =.*/blobGatewayApp = '$blob_gateway_appid',/g" webui/src/const/index.ts
sed -i "s/cpRegistryApp =.*/cpRegistryApp = '$cp_registry_appid',/g" webui/src/const/index.ts
sed -i "s/copilotCpuApp =.*/copilotCpuApp = '$copilot_cpu_appid',/g" webui/src/const/index.ts
sed -i "s/copilotGpuApp =.*/copilotGpuApp = '$copilot_gpu_appid',/g" webui/src/const/index.ts
sed -i "s/illustratorCpuApp =.*/illustratorCpuApp = '$illustrator_cpu_appid',/g" webui/src/const/index.ts
sed -i "s/illustratorGpuApp =.*/illustratorGpuApp = '$illustrator_gpu_appid'/g" webui/src/const/index.ts

sed -i "s/export const appDeployChain =.*/export const appDeployChain = '$app_deploy_chain'/g" webui/src/const/index.ts
sed -i "s/export const appDeployOwner =.*/export const appDeployOwner = '$app_deploy_owner'/g" webui/src/const/index.ts

for i in `seq 0 $EXTRA_WALLET_NUMBER`; do
  run_service $i &
done

trap cleanup INT
read -p "  Press any key to exit"
print $'\U01f499' $LIGHTGREEN " Exit ..."

cleanup

