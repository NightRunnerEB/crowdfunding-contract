
# Crowdfunding Contract

## Описание

Этот проект представляет собой смарт-контракт для краудфандинга на блокчейне MultiversX. CLI для взаимодействия с контрактом предоставляет возможности для деплоя контракта, финансирования, получения данных о контракте, а также других действий на тестовой сети блокчейна - Devnet.

## Предварительные требования

Перед началом работы убедитесь, что установлены нужные инструменты:

### **MultiversX SDK (`mxpy`)**:
   Для взаимодействия с сетью MultiversX и управления контрактами необходимо установить multiversx-sdk-cli. Перед установкой mxpy убедитесь, что у вас работает рабочая среда Python 3. Вам понадобится Python 3.8 или более поздняя версия в Linux или MacOS.

   - Установите `mxpy`:
     ```bash
     pipx install multiversx-sdk-cli --force
     ```

   - Проверьте, что `mxpy` установлен:
     ```bash
     mxpy --version
     ```

## Развертывание и использование

### 1. Клонирование репозитория

Клонируйте репозиторий проекта на свой компьютер:

```bash
git clone https://github.com/your-repo/crowdfunding-contract.git
cd crowdfunding-contract
```

### 2. Сборка проекта

Для сборки проекта выполните:

```bash
sc-meta all build
```

### 3. Деплой смарт-контракта

Чтобы развернуть смарт-контракт в сети MultiversX (например, на devnet), выполните команду:

```bash
cd interactor
```

```bash
cargo run -- deploy --target <целевое-значение> --deadline <дедлайн>
```
После этой команды в интеракторе мы получим sc deploy tx hash и deploy address.
Первое это хэш (уникальный идентификатор) транзакции, которая развертывает смарт-контракт и возвращает его адрес(deploy address).

Пример:

```bash
cargo run -- deploy --target 1000 --deadline 123456
```

После успешного деплоя будет выведен новый адрес контракта.

### 4. Финансирование контракта

Для финансирования контракта выполните:

```bash
cargo run -- fund --amount <сумма>
```

Пример:

```bash
cargo run -- fund --amount 500
```

### 5. Получение данных о контракте

Для получения информации о контракте используйте следующие команды:

- Получить целевое значение:
  ```bash
  cargo run -- get-target
  ```

- Получить дедлайн:
  ```bash
  cargo run -- get-deadline
  ```

- Получить информацию о депозите:
  ```bash
  cargo run -- get-deposit --donor <адрес-донатора>
  ```

- Пример:

   ```bash
  cargo run -- get-deposit --donor erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th
   ```

- Узнать текущие собранные средства:
  ```bash
  cargo run -- get-currentFunds
  ```

- Узнать статус контракта:
  ```bash
  cargo run -- status
  ```

### 6. Запрос выплаты средств (claim)

Когда краудфандинговая цель достигнута, вы можете запросить выплату средств, выполнив команду:

```bash
cargo run -- claim --address <адрес получателя>
```

## Дополнительные инструкции для разработчиков

- **Работа с сетью**: Вы можете переключаться между devnet и mainnet сетями, изменяя переменную `GATEWAY` в коде интерактора.

- **Модульные тесты и сценарии

Для разработчиков предусмотрена возможность запуска модульных тестов на виртуальных машинах **GoVM** и **RustVM**. Эти виртуальные машины позволяют локально тестировать смарт-контракты без необходимости деплоя в публичную сеть, что существенно ускоряет процесс тестирования и отладки.

#### Запуск тестов на GoVM и RustVM

Модульные, сценарные и интеграционные тесты можно запускать с помощью команды **`mxpy contract test`**, которая выполнит тестирование на обеих виртуальных машинах: GoVM и RustVM.
