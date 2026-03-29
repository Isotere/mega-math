CRATE_TARGETS := build check test new-crate
PROFILE ?= debug
CARGO_PROFILE_FLAG := $(if $(filter release,$(PROFILE)),--release,)

ifneq ($(filter $(CRATE_TARGETS),$(firstword $(MAKECMDGOALS))),)
  CRATE := $(wordlist 2,2,$(MAKECMDGOALS))
  ifdef CRATE
    $(eval $(CRATE):;@:)
  endif
endif

.PHONY: lint-all fmt-all build-all test-all clean $(CRATE_TARGETS)

# ------------------------------------------------------------------------
# 🧹 Очистка: make clean
# ------------------------------------------------------------------------
clean:
	@echo "🧹 Очистка артефактов сборки..."
	@cargo clean
	@echo "✅ Директория target/ удалена."

# ------------------------------------------------------------------------
# 🔍 Линтер: make lint-all
# ------------------------------------------------------------------------
lint-all:
	@echo "🔍 Запуск Clippy на весь workspace..."
	@cargo clippy --workspace -- -D warnings
	@echo "✅ Линтер пройден без ошибок."

# ------------------------------------------------------------------------
# 🎨 Форматирование: make fmt-all
# ------------------------------------------------------------------------
fmt-all:
	@echo "🎨 Форматирование всех крейтов..."
	@cargo fmt --all
	@echo "✅ Форматирование завершено."

# ------------------------------------------------------------------------
# 🏗 Сборка ВСЕГО workspace: make build-all [PROFILE=release]
# ------------------------------------------------------------------------
build-all:
	@echo "📦 Сборка всех крейтов ($(PROFILE))..."
	@cargo build --workspace $(CARGO_PROFILE_FLAG)
	@echo "✅ Все крейты собраны ($(PROFILE))."

# ------------------------------------------------------------------------
# 🧪 Тесты ВСЕГО workspace: make test-all
# ------------------------------------------------------------------------
test-all:
	@echo "🧪 Запуск тестов для всех крейтов..."
	@cargo test --workspace
	@echo "✅ Все тесты пройдены."

# ------------------------------------------------------------------------
# 🎯 Сборка крейта: make build <crate> [PROFILE=release]
# ------------------------------------------------------------------------
build:
ifndef CRATE
	$(error Crate name is required. Usage: make build <crate>)
endif
	@echo "🔧 Сборка mega-math-$(CRATE) ($(PROFILE))..."
	@cargo build -p mega-math-$(CRATE) $(CARGO_PROFILE_FLAG)
	@echo "✅ mega-math-$(CRATE) собран ($(PROFILE))."

# ------------------------------------------------------------------------
# 🔎 Проверка крейта: make check <crate>
# ------------------------------------------------------------------------
check:
ifndef CRATE
	$(error Crate name is required. Usage: make check <crate>)
endif
	@echo "🔎 Проверка mega-math-$(CRATE)..."
	@cargo check -p mega-math-$(CRATE)
	@echo "✅ mega-math-$(CRATE) — ошибок нет."

# ------------------------------------------------------------------------
# 🧪 Тесты крейта: make test <crate>
# ------------------------------------------------------------------------
test:
ifndef CRATE
	$(error Crate name is required. Usage: make test <crate>)
endif
	@echo "🧪 Запуск тестов mega-math-$(CRATE)..."
	@cargo test -p mega-math-$(CRATE)
	@echo "✅ Тесты mega-math-$(CRATE) пройдены."

# ------------------------------------------------------------------------
# 🆕 Новый крейт: make new-crate <crate> DESCRIPTION="..."
# ------------------------------------------------------------------------
new-crate:
ifndef CRATE
	$(error Crate name is required. Usage: make new-crate <crate> DESCRIPTION="...")
endif
	@echo "🆕 Создание крейта mega-math-$(CRATE)..."
	@cp -r template crates/$(CRATE)
	@sed -i '' 's/{{name}}/$(CRATE)/g' crates/$(CRATE)/Cargo.toml
	@sed -i '' 's/{{description}}/$(or $(DESCRIPTION),TODO)/g' crates/$(CRATE)/Cargo.toml
	@echo "✅ Крейт создан: crates/$(CRATE)/"
