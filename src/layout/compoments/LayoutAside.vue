<template>
  <div>
    <TinyLayout
        :style="{height: height + 'px', padding: '10px 0px', textAlign: 'center', backgroundColor: '#FFFFFF', borderRightWidth: '1px', borderRightStyle: 'groove'}">
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
                :style="{height: height + 'px', width: width + 'px', borderRightWidth: '1px', borderRightStyle: 'groove'}"
                @node-click="handlerNodeClick">
        <template #operation="{ node }">
          <div v-if="node.data.draft"
               class="operation-slot">
            <TinyTooltip effect="dark"
                         content="保存">
              <IconSave @click="handlerSave(node.data)"></IconSave>
            </TinyTooltip>
          </div>
          <TinyTooltip effect="dark"
                       content="删除">
            <IconDel @click="handlerDelete(node.data)"></IconDel>
          </TinyTooltip>
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
import {IconDel, IconSave} from '@opentiny/vue-icon'
import {invoke} from '@tauri-apps/api/tauri'
import {Note} from "../../model/note.ts";
import {Response} from "../../model/response.ts";

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
    IconSave: IconSave(),
    IconDel: IconDel()
  },
  props: {
    height: {
      type: Number
    },
    width: {
      type: Number
    },
    node: {
      type: Object as () => Note
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
          .then(value => {
            const response = value as Response
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
      const tree = this.$refs.tree as any
      tree.append(note)
      tree.setCurrentKey(note.id)
      this.$emit('onClick', note)
    },
    handlerSave(note: Note) {
      if (!note.modify) {
        note.id = 0
      }
      const cmd = note.modify ? 'update_note' : 'create_note'
      invoke(cmd, {note: note})
          .then(value => {
            const response = value as Response
            if (response.code === 200 && response.data) {
              const message = note.modify ? `保存 [ ${note.title} ] 成功` : `新建 [ ${note.title} ] 成功`
              Notify({
                type: 'success',
                message: message,
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
    },
    handlerNodeClick(data: Note) {
      this.$emit('onClick', data)
    },
    handlerDelete(note: Note) {
      if (note.id.toString().startsWith('custom_')) {
        const tree = this.$refs.tree as any
        tree.remove(note.id)
      } else {
        invoke('delete_note', {id: note.id})
            .then(value => {
              const response = value as Response
              if (response.code === 200 && response.data) {
                Notify({
                  type: 'success',
                  message: `删除 [ ${note.title} ] 成功`,
                  position: 'top',
                  title: '提示',
                  duration: 1000
                })
                this.$emit('onDelete', note)
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
  },
  watch: {
    node() {
      const tree = this.$refs.tree as any
      if (this.node) {
        tree.setCurrentKey(this.node.id)
      } else {
        tree.setCurrentKey(undefined)
      }
    },
    'node.modify': {
      handler() {
        if (this.node) {
          this.node.draft = true
        }
      }
    }
  }
});
</script>
