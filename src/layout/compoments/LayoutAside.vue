<template>
  <div>
    <TinyLayout
        :style="{height: height + 'px', padding: '10px 0px', textAlign: 'center', backgroundColor: '#FFFFFF'}">
      <TinyDropdown split-button
                    type="primary"
                    @button-click="handlerButtonClick">
        新建笔记
        <template #dropdown>
        </template>
      </TinyDropdown>
      <TinyDivider :style="{margin: '12px 0px 0px 0px'}">
      </TinyDivider>
      <TinyTree ref="tree"
                node-key="id"
                :data="data"
                :show-line="true"
                :style="{height: height + 'px', width: width + 'px'}">
        <template #operation="{ node }">
          <div v-if="node.data.draft"
               class="operation-slot">
            <TinyTooltip effect="dark"
                         content="保存">
              <IconSave @click="handlerSave(node.data)"></IconSave>
            </TinyTooltip>
          </div>
        </template>
      </TinyTree>
    </TinyLayout>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue"
import {
  Divider as TinyDivider,
  Dropdown as TinyDropdown,
  DropdownItem as TinyDropdownItem,
  DropdownMenu as TinyDropdownMenu,
  Layout as TinyLayout,
  Notify,
  Tooltip as TinyTooltip,
  Tree as TinyTree
} from '@opentiny/vue'
import {IconSave} from '@opentiny/vue-icon'
import {invoke} from '@tauri-apps/api/tauri'
import {Note} from "../../model/note.ts";

export default defineComponent({
  name: 'LayoutAside',
  components: {
    TinyLayout,
    TinyDropdown,
    TinyDropdownMenu,
    TinyDropdownItem,
    TinyDivider,
    TinyTooltip,
    TinyTree,
    IconSave: IconSave()
  },
  props: {
    height: {
      type: Number
    },
    width: {
      type: Number
    }
  },
  data() {
    return {
      data: []
    }
  },
  created() {
    this.handlerInitialize()
  },
  methods: {
    handlerInitialize() {
      invoke('get_notes')
          .then(response => {
            if (response.code === 200) {
              this.data = response.data
            } else {
              Notify({
                type: 'error',
                message: response.message,
                position: 'top',
                title: '错误',
                duration: 1000
              })
            }
          })
    },
    handlerButtonClick() {
      const key = Date.now().toString()
      const title = '新笔记' + key
      const note: Note = {
        id: 'custom_' + key,
        key: key,
        title: title,
        label: title,
        name: key,
        editor: 'Markdown',
        content: '',
        draft: true
      }
      this.$refs.tree.append(note)
      this.$refs.tree.setCurrentKey(note.id)
      this.$emit('onClick', note)
    },
    handlerSave(note: Note) {
      note.id = 0
      invoke('create_note', {note: note})
          .then(response => {
            if (response.code === 200 && response.data) {
              Notify({
                type: 'success',
                message: `保存 [ ${note.title} ] 成功`,
                position: 'top',
                title: '提示',
                duration: 1000
              })
              this.handlerInitialize()
            } else {
              Notify({
                type: 'error',
                message: response.message,
                position: 'top',
                title: '错误',
                duration: 1000
              })
            }
          })
    }
  }
});
</script>
