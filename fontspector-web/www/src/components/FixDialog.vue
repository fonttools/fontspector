<script setup lang="ts">
import { computed, watch } from 'vue';
import {
    DialogField,
    isChoiceDialogFieldType,
    MoreInfoReply,
    MoreInfoReplyValue,
    MoreInfoRequest,
} from '../types';

const props = withDefaults(defineProps<{
    request: MoreInfoRequest;
    modelValue?: MoreInfoReply;
}>(), {
    modelValue: () => ({}),
});

const emit = defineEmits<{
    (e: 'update:modelValue', value: MoreInfoReply): void;
    (e: 'update:valid', value: boolean): void;
}>();

const replies = computed(() => props.modelValue || {});

function currentValue(field: DialogField): MoreInfoReplyValue {
    return replies.value[field.key] ?? null;
}

function isFieldValid(field: DialogField): boolean {
    const value = currentValue(field);

    if (isChoiceDialogFieldType(field.field_type)) {
        return typeof value === 'string' && value.trim().length > 0;
    }

    if (field.field_type === 'Text') {
        return typeof value === 'string' && value.trim().length > 0;
    }

    if (field.field_type === 'Number') {
        return typeof value === 'number' && Number.isFinite(value);
    }

    if (field.field_type === 'Boolean') {
        return typeof value === 'boolean';
    }

    return false;
}

const valid = computed(() => props.request.every((field) => isFieldValid(field)));

watch(valid, (value) => {
    emit('update:valid', value);
}, { immediate: true });

function updateField(key: string, value: MoreInfoReplyValue) {
    emit('update:modelValue', {
        ...replies.value,
        [key]: value,
    });
}
</script>

<template>
    <div class="fix-dialog card mt-3">
        <div class="card-body">
            <h6 class="card-title">More information needed</h6>
            <p class="card-text text-body-secondary">
                This fix needs a few answers before it can be applied.
            </p>

            <div v-for="field in request" :key="field.key" class="mb-3">
                <label :for="`fix-dialog-${field.key}`" class="form-label prompt-label">{{ field.prompt }}</label>

                <select v-if="isChoiceDialogFieldType(field.field_type)" :id="`fix-dialog-${field.key}`"
                    class="form-select" :value="typeof currentValue(field) === 'string' ? currentValue(field) : ''"
                    @change="(event) => updateField(field.key, (event.target as HTMLSelectElement).value || null)">
                    <option value="">Select an option…</option>
                    <option v-for="choice in field.field_type.Choice" :key="choice.value" :value="choice.value">
                        {{ choice.description }}
                    </option>
                </select>

                <input v-else-if="field.field_type === 'Text'" :id="`fix-dialog-${field.key}`" type="text"
                    class="form-control" :value="typeof currentValue(field) === 'string' ? currentValue(field) : ''"
                    @input="(event) => updateField(field.key, (event.target as HTMLInputElement).value || null)" />

                <input v-else-if="field.field_type === 'Number'" :id="`fix-dialog-${field.key}`" type="number"
                    class="form-control" :value="typeof currentValue(field) === 'number' ? currentValue(field) : ''"
                    @input="(event) => {
                        const raw = (event.target as HTMLInputElement).value;
                        updateField(field.key, raw === '' ? null : Number(raw));
                    }" />

                <div v-else-if="field.field_type === 'Boolean'" class="form-check mt-2">
                    <input :id="`fix-dialog-${field.key}`" type="checkbox" class="form-check-input"
                        :checked="Boolean(currentValue(field))"
                        @change="(event) => updateField(field.key, (event.target as HTMLInputElement).checked)" />
                    <label :for="`fix-dialog-${field.key}`" class="form-check-label">Yes</label>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.fix-dialog {
    border-color: color-mix(in srgb, var(--bs-primary), transparent 75%);
}

.prompt-label {
    white-space: pre-line;
}
</style>
