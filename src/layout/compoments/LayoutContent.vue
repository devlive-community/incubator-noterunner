<template>
  <div>
    <TinyLayout :style="{backgroundColor: '#FFFFFF', height: height + 'px'}">
      <TinyTabs v-model="activeTab"
                tab-style="card"
                :with-close="true"
                @close="handlerClose"
                @click="handlerClick">
        <TinyTabItem v-for="item in tabs"
                     :key="item.key"
                     :style="{margin: 0, padding: 0}"
                     :name="item.name">
          <template #title>
            <div>
              <TinyBadge v-if="item.modify"
                         is-dot>
                {{ item.title }}
              </TinyBadge>
              <span v-else>{{ item.title }}</span>
            </div>
          </template>
          <MarkdownEditor v-if="item.editor === 'Markdown'"
                          :style="{height: (height as number - 75) + 'px'}"
                          :content="item.content"
                          :key="item.key"
                          @onChange="handlerChange($event, item)">
          </MarkdownEditor>
        </TinyTabItem>
      </TinyTabs>
    </TinyLayout>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue"
import {Badge as TinyBadge, Layout as TinyLayout, TabItem as TinyTabItem, Tabs as TinyTabs} from '@opentiny/vue'
import MarkdownEditor from "../../components/MarkdownEditor.vue";
import {Note} from "../../model/note.ts";

export default defineComponent({
  name: 'LayoutContent',
  components: {MarkdownEditor, TinyLayout, TinyTabs, TinyTabItem, TinyBadge},
  props: {
    height: {
      type: Number
    },
    note: {
      type: Object as () => Note
    },
    deletedNote: {
      type: Object as () => Note
    }
  },
  data() {
    return {
      activeTab: '',
      activeNote: this.note,
      tabs: Array<Note>()
    }
  },
  methods: {
    handlerChange(content: string, note: Note) {
      note.content = content
      this.activeNote = note
      this.$emit('onClick', this.note)
    },
    handlerClose(name: string) {
      const index = this.tabs.findIndex(item => item.name === name)
      if (this.tabs.length > 1) {
        const key = this.tabs[index === 0 ? index + 1 : index - 1].name
        this.activeTab = key
      } else {
        this.activeTab = this.tabs[0].name
      }
      this.tabs.splice(index, 1)
      this.handlerClick({name: this.activeTab})
    },
    handlerClick(tab: any) {
      const index = this.tabs.findIndex(item => item.name === tab.name)
      this.$emit('onClick', this.tabs[index])
    }
  },
  watch: {
    note() {
      const hasValue = this.tabs.filter(item => item.name === this.note?.name)
      if (hasValue.length === 0) {
        this.tabs.push(<Note>this.note)
      }
      const note = this.note as Note
      this.activeTab = note.key
    },
    // If a content modification is found, the current data is marked as modified
    "activeNote.content": {
      handler() {
        const note = this.activeNote as Note
        if (!note.id.toString().startsWith('custom_')) {
          note.modify = true
        }
        this.$emit('onClick', this.note)
      }
    },
    deletedNote() {
      if (this.deletedNote) {
        this.handlerClose(this.deletedNote?.name)
      }
    }
  }
});
</script>
<style scoped>
.tiny-tabs__content {
  padding: 0;
}
</style>
