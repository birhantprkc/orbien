<script setup lang="ts">
import {useI18n} from "vue-i18n";
import {pickFile, type FileFilter} from "@/api/desktop";
import type {MessageSchema} from "@/i18n";

const model = defineModel<string>({default: ""});

const props = withDefaults(
    defineProps<{
      label: string;
      placeholder?: string;
      filters?: FileFilter[];
    }>(),
    {
      placeholder: "",
      filters: () => [
        {name: "Certificate / Key", extensions: ["pem", "crt", "cer", "key", "pub"]},
        {name: "All files", extensions: ["*"]},
      ],
    },
);

const {t} = useI18n<{ message: MessageSchema }>();

async function browse() {
  try {
    const path = await pickFile({
      title: props.label,
      filters: props.filters,
    });
    if (path) model.value = path;
  } catch {
  }
}
</script>

<template>
  <label class="field path-field">
    <span>{{ label }}</span>
    <div class="path-row">
      <input v-model="model" :placeholder="placeholder"/>
      <button class="browse-btn" type="button" @click="browse">
        {{ t("common.browse") }}
      </button>
    </div>
  </label>
</template>

<style scoped>
.field {
  display: grid;
  gap: 0.4rem;
  min-width: 0;
}

.field > span {
  font-weight: 600;
  font-size: 0.9rem;
  color: var(--text);
}

.path-row {
  display: flex;
  gap: 0.45rem;
  min-width: 0;
}

.path-row input {
  border: 1px solid var(--line);
  border-radius: var(--radius);
  padding: 0.7rem 0.8rem;
  background: #fff;
  color: var(--text);
  width: 100%;
  min-width: 0;
}

.path-row input:focus {
  outline: none;
  border-color: rgba(59, 130, 246, 0.55);
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.browse-btn {
  flex-shrink: 0;
  border: 1px solid var(--line);
  border-radius: var(--radius);
  padding: 0.7rem 0.85rem;
  background: transparent;
  color: var(--accent);
  font: inherit;
  font-weight: 600;
  font-size: 0.85rem;
  cursor: pointer;
}

.browse-btn:hover {
  background: var(--accent-soft);
}
</style>
