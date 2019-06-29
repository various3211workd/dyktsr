# dyktsr

## dyktsrって何？ What's dyktsr?

![キャプチャ](https://user-images.githubusercontent.com/43775946/60386182-e9eadd00-9acc-11e9-8ce7-ead3fc3c6ff3.PNG)

dyktsrはサンバのリズムを知ってるかい？の略です。  
入力された文字列をサンバのリズムに変換します。

'dyktsr' stands for 'do_you_know_the_samba_rhythm?'.  
Convert input string to samba rhythm

## 使い方 Usage

### 1. 基本的なサンバのリズム  Plane samba rhythm  
あなたにとって特別な2文字を入れるとコンソールがサンバのリズムを出力します。

The console will output the samba rhythm when you enter two characters that are special to you.

```powershell
PS>./dyktsr ホイ
ホホホイ🙏   ホホホイ🙏   ホホホイホイ🙏
```

***

### 2. 色々サンバのリズム  Any samba rhythm
オプションとして`i`をつけるとアイコンの見た目を指定できます。

You can specify the appearance of the icon by adding `i` as an option.

```powershell
PS>./dyktsr カニ -i 🦀
カカカニ🦀   カカカニ🦀   カカカニカニ🦀
```

***

### 3. プエルトリコ  Puerto Rico
さらにオプションとして`p`をつけるとプエルトリコモードでリズムを刻みます。

If you add `p` as an option, the rhythm will be set in Puerto Rico mode.

```powershell
PS>./dyktsr ウキ -pi 🐵
ウキウキ🐵   ウウウウキ🐵   ウキウキウキウウキウウキ🐵   ウキ🐵
```

***

#### よいサンバのリズムを！ Have a nice samba rhythm!

## ビルドとインストール Build and Install

### for Windows
move release tab and donwload

### for Mac
1. Download source code.
2. Edit file name on `.cargo/temp_config` => `.cargo/config`
3. build

### for Linux
1. Download source code.
2. Edit file name on `.cargo/temp_config` => `.cargo/config`
3. build
