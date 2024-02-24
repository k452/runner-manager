# runner-manager

## 概要

自宅で稼働しているサーバーへの操作を行う際に利用する self-hosted runner を job 毎にコンテナで起動し、job 間で使い回さないようにする仕組みを実現するためのアプリケーション  
本アプリケーションをビルドしたバイナリを runner が稼働するサーバー(Raspberry Pi)上の systemd で常時起動させて使用することを想定している

## アーキテクチャ構成図

![architecture](/docs/architecture.drawio.png)

### 処理の流れ

1. まず管理したい Proxmox サーバーの状態を記述した Terraform のコードを github へ push
2. github 側で事前に main ブランチへの push 時に webhook が発火して API Gateway の所定のエンドポイントへ POST するように設定
3. webhook からの payload を受け取った API Gateway は Lambda へそのまま proxy
   1. ここで API Gateway を経由しているのは Lambda を直接叩いた場合、初回実行に時間がかかりレスポンスが遅延した場合に webhook からの送信がタイムアウトにより失敗してしまうため
4. Lambda は SQS へ payload を送信
5. Raspberry Pi 上の runner-manager は systemd で常時起動しており、定期的に SQS にキューが溜まっていないか確認しており、溜まっていたら取得して処理を実行する
6. Raspberry Pi 上で起動している Docker Registry に事前にビルドした runner 用イメージを格納しておき、それを利用してコンテナを起動する
   1. matrix 等で複数コンテナが起動する場合もある
7. 起動したコンテナが github に runner として登録され、必要な処理が実行され、自宅 NW 内の Proxmox サーバーがプロビジョニングされる

# ポイント

- 以下の２点により job 毎に異なるコンテナ上で処理が実行される仕組みを実現している
  - runner の起動時に ephemeral オプションを指定することによって、処理が終わった際に自動的に runner の登録が解除されるようになっている
  - コンテナ起動時に rm オプションを指定することによって処理が終わった際にコンテナが破棄されるようになっている
- この仕組みを提供するための[リポジトリ](https://github.com/philips-labs/terraform-aws-github-runner)が存在しているが、対象が自宅 NW であることとコンテナをスケーリングするものではなかったため採用を見送った
- また、k8s を利用してスケーリングするものも用意されていたが、こちらも当然今回のユースケースとは乖離していたため採用を見送った
- 基本的な仕組みは[こちらの記事](https://engineers.ntt.com/entry/2022/11/04/084857)を参考にしており、自宅 NW に webhook から直接アクセスするのを避けるために SQS を中継地点とする仕組みに改変している
