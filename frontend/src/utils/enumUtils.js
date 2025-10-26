export function enumToLabel(enumList, value) {
  const item = enumList.find(e => e.value === value)
  return item ? item.label : String(value)
}

export function labelToEnum(enumList, label) {
  const item = enumList.find(e => e.label === label)
  return item ? item.value : label
}