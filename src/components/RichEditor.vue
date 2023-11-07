<template>
  <div :style="{height: height + 'px', maxHeight: height + 'px'}"
       :id="elementId">
  </div>
</template>

<script lang="ts">
import {defineComponent, onMounted, onUnmounted, ref} from 'vue'
import Vditor from 'vditor'
import 'vditor/dist/index.css'

export default defineComponent({
  name: "rich-editor",
  components: {},
  props: {
    height: {
      type: Number
    },
    content: {
      type: String
    }
  },
  setup(props, {emit}) {
    const elementId = `editor-${Math.random().toString(36).substr(2, 9)}`
    let richEditor = ref<Vditor | null>(null)

    onMounted(() => {
      richEditor.value = new Vditor(elementId, {
        height: props.height,
        value: props.content,
        toolbar: ['emoji', 'headings', 'bold', 'italic', 'strike', '|', 'line', 'quote', 'list', 'ordered-list', 'check', 'outdent', 'indent', 'code', 'inline-code', 'insert-after', 'insert-before', 'undo', 'redo', 'upload', 'link', 'table', 'record', 'both', 'preview', 'fullscreen', 'outline', 'code-theme', 'content-theme', 'export', 'br'],
        cache: {enable: false},
        toolbarConfig: {pin: true},
        counter: {enable: false},
        mode: 'wysiwyg',
        input(value: string) {
          emit('onChange', value)
        }
      })
    })

    onUnmounted(() => {
      if (richEditor.value) {
        richEditor = ref<Vditor | null>(null)
      }
    })

    return {elementId}
  }
})
</script>
