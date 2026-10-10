{{#title MIFARE Classic 1K Access Bits Calculator for ESP32}}

# Калькулятор битов доступа MIFARE Classic 1K

<style>
.nested-table {
    display: none;
    margin-top: 10px;
    border: 1px solid #ccc;
    padding: 10px;
}

.nested-table table {
    width: 100%;
    border-collapse: collapse;
}

.nested-table th, .nested-table td {
    border: 1px solid #ccc;
    padding: 5px;
    text-align: center;
}

.nested-table tr:hover{
    cursor: pointer;
}

.nested-table tr:hover{
    background:#30521b;
    color: white;
}

.edit-button {
    cursor: pointer;
    padding: 5px 10px;
}

.selected {
    background-color: #f0f0f0;
}

.selected-row {
    background-color: #d3f4d7;
}
</style>

Декодирование:
Вы можете изменить «Биты доступа», и таблицы блоков данных и трейлера сектора обновятся автоматически.

Кодирование:
Нажмите кнопку «Изменить» в каждой строке таблицы, чтобы выбрать нужные условия доступа. Это обновит биты доступа.

> [!Caution]
> Запись неверного значения в биты условий доступа может сделать сектор недоступным.


**Биты доступа**
<div style="display: flex; align-items: center;">
  <input type="text" 
         style="font-size:16px; height: 40px;" 
         name="access_bits" 
         id="access-bits" 
         value="FF0780"/>
  <div id="error-box"   style="ddisplay: none;  color: #FF2800; font-size: 18px; margin-left: 10px; font-weight:bold">
  </div>
</div>


**Условия доступа для блоков данных:**

<table border="1" id="db-table">
  <thead>
    <tr>
      <th>Блок</th>
      <th>C1Y</th>
      <th>C2Y</th>
      <th>C3Y</th>
      <th>Чтение</th>
      <th>Запись</th>
      <th>Инкремент</th>
      <th>Декремент/Передача/Восстановление</th>
      <th>Примечания</th>
      <th>Действие</th>
    </tr>
  </thead>
  <tbody>
    <tr data-row="1" id="db-row0">
      <td>Блок 0</td>
      <td>0</td>
      <td>0</td>
      <td>0</td>
      <td>ключ A|B</td>
      <td>ключ A|B</td>
      <td>ключ A|B</td>
      <td>ключ A|B</td>
      <td>Конфигурация по умолчанию</td>
      <td><button class="edit-button" onclick="toggleNestedTable(this, 'nested-table',10)">Изменить</button></td>
    </tr>
    <tr data-row="2" id="db-row1">
      <td>Блок 1</td>
      <td>0</td>
      <td>0</td>
      <td>0</td>
      <td>ключ A|B</td>
      <td>ключ A|B</td>
      <td>ключ A|B</td>
      <td>ключ A|B</td>
      <td>Конфигурация по умолчанию</td>
      <td><button class="edit-button" onclick="toggleNestedTable(this, 'nested-table',10)">Изменить</button></td>
    </tr>
    <tr data-row="3" id="db-row2">
      <td>Блок 2</td>
      <td>0</td>
      <td>0</td>
      <td>0</td>
      <td>ключ A|B</td>
      <td>ключ A|B</td>
      <td>ключ A|B</td>
      <td>ключ A|B</td>
      <td>Конфигурация по умолчанию</td>
      <td><button class="edit-button" onclick="toggleNestedTable(this, 'nested-table',10)">Изменить</button></td>
    </tr>
  </tbody>
</table>


<!-- Nested Table Template -->
<div class="nested-table" id="nested-table">
  <table border="1">
    <thead>
      <tr>
        <th>C1Y</th>
        <th>C2Y</th>
        <th>C3Y</th>
        <th>Чтение</th>
        <th>Запись</th>
        <th>Инкремент</th>
        <th>Декремент/Передача/Восстановление</th>
        <th>Примечания</th>
      </tr>
    </thead>
    <tbody>
      <tr onclick="selectRow(this)">
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>ключ A|B</td>
        <td>ключ A|B</td>
        <td>ключ A|B</td>
        <td>ключ A|B</td>
        <td>Конфигурация по умолчанию</td>
      </tr>
      <tr onclick="selectRow(this)">
        <td>0</td>
        <td>1</td>
        <td>0</td>
        <td>ключ A|B</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>блок чтения/записи</td>
      </tr>
      <tr onclick="selectRow(this)">
        <td>1</td>
        <td>0</td>
        <td>0</td>
        <td>ключ A|B</td>
        <td>ключ B</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>блок чтения/записи</td>
      </tr>
      <tr onclick="selectRow(this)">
        <td>1</td>
        <td>1</td>
        <td>0</td>
        <td>ключ A|B</td>
        <td>ключ B</td>
        <td>ключ B</td>
        <td>ключ A|B</td>
        <td>блок значения</td>
      </tr>
      <tr onclick="selectRow(this)">
        <td>0</td>
        <td>0</td>
        <td>1</td>
        <td>ключ A|B</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>ключ A|B</td>
        <td>блок значения</td>
      </tr>
      <tr onclick="selectRow(this)">
        <td>0</td>
        <td>1</td>
        <td>1</td>
        <td>ключ B</td>
        <td>ключ B</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>блок чтения/записи</td>
      </tr>
      <tr onclick="selectRow(this)">
        <td>1</td>
        <td>0</td>
        <td>1</td>
        <td>ключ B</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>блок чтения/записи</td>
      </tr>
      <tr onclick="selectRow(this)">
        <td>1</td>
        <td>1</td>
        <td>1</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>блок чтения/записи</td>
      </tr>
    </tbody>
  </table>
</div>

**Условия доступа для трейлера сектора (блок 3):**

<table border="1" id="st-table">
  <thead>
    <tr>
        <th>C1<sub>3</sub></th>
        <th>C2<sub>3</sub></th>
        <th>C3<sub>3</sub></th>
        <th>Чтение ключа A</th>
        <th>Запись ключа A</th>
        <th>Чтение битов доступа</th>
        <th>Запись битов доступа</th>
        <th>Чтение ключа B</th>
        <th>Запись ключа B</th>
        <th>Примечания</th>
        <th>Действие</th>
    </tr>
  </thead>
  <tbody>
    <tr data-row="1" id="st-row0"> 
        <td>0</td>
        <td>0</td>
        <td>1</td>
        <td>никогда</td>
        <td>ключ A</td>
        <td>ключ A</td>
        <td>ключ A</td>
        <td>ключ A</td>
        <td>ключ A</td>
        <td>Ключ B можно прочитать; Конфигурация по умолчанию</td>
        <td><button class="edit-button" onclick="toggleNestedTable(this, 'nested-st', 11)">Изменить</button></td>
    </tr>
  </tbody>
</table>

<div class="nested-table" id="nested-st">
  <table border="1">
    <thead>
      <tr>
        <th>C1<sub>3</sub></th>
        <th>C2<sub>3</sub></th>
        <th>C3<sub>3</sub></th>
        <th>Чтение ключа A</th>
        <th>Запись ключа A</th>
        <th>Чтение битов доступа</th>
        <th>Запись битов доступа</th>
        <th>Чтение ключа B</th>
        <th>Запись ключа B</th>
        <th>Примечания</th>
      </tr>
    </thead>
    <tbody>
      <tr onclick="selectStRow(this)">
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>никогда</td>
        <td>ключ A</td>
        <td>ключ A</td>
        <td>никогда</td>
        <td>ключ A</td>
        <td>ключ A</td>
        <td>Ключ B можно прочитать</td>
      </tr>
      <tr onclick="selectStRow(this)">
        <td>0</td>
        <td>1</td>
        <td>0</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>ключ A</td>
        <td>никогда</td>
        <td>ключ A</td>
        <td>никогда</td>
        <td>Ключ B можно прочитать</td>
      </tr>
      <tr onclick="selectStRow(this)">
        <td>1</td>
        <td>0</td>
        <td>0</td>
        <td>никогда</td>
        <td>ключ B</td>
        <td>ключ A|B</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>ключ B</td>
        <td></td>
      </tr>
      <tr onclick="selectStRow(this)">
        <td>1</td>
        <td>1</td>
        <td>0</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>ключ A|B</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>никогда</td>
        <td></td>
      </tr>
      <tr onclick="selectStRow(this)">
        <td>0</td>
        <td>0</td>
        <td>1</td>
        <td>никогда</td>
        <td>ключ A</td>
        <td>ключ A</td>
        <td>ключ A</td>
        <td>ключ A</td>
        <td>ключ A</td>
        <td>Ключ B можно прочитать; Конфигурация по умолчанию</td>
      </tr>
      <tr onclick="selectStRow(this)">
        <td>0</td>
        <td>1</td>
        <td>1</td>
        <td>никогда</td>
        <td>ключ B</td>
        <td>ключ A|B</td>
        <td>ключ B</td>
        <td>никогда</td>
        <td>ключ B</td>
        <td></td>
      </tr>
      <tr onclick="selectStRow(this)">
        <td>1</td>
        <td>0</td>
        <td>1</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>ключ A|B</td>
        <td>ключ B</td>
        <td>никогда</td>
        <td>никогда</td>
        <td></td>
      </tr>
      <tr onclick="selectStRow(this)">
        <td>1</td>
        <td>1</td>
        <td>1</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>ключ A|B</td>
        <td>никогда</td>
        <td>никогда</td>
        <td>никогда</td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>



<script>
  function toggleNestedTable(button, tmplId, colSpan) {
    const row = button.closest('tr');
    const rowId = row.id;
    // console.log(rowId);

    const table = row.closest('table');
    let nestedTable = row.nextElementSibling;

    // Check if nested table already exists, otherwise create a new one
    if (!nestedTable || !nestedTable.classList.contains('nested-table')) {
      nestedTable = document.createElement('tr');
      const td = document.createElement('td');
      td.colSpan = colSpan;
      nestedTable.classList.add('nested-table');
      nestedTable.setAttribute('data-parent', rowId); 
      td.innerHTML = document.getElementById(tmplId).innerHTML; 
      nestedTable.appendChild(td);
       // Insert the nested table below the row
      row.insertAdjacentElement('afterend', nestedTable);
    }

    // Toggle visibility of the nested table
    nestedTable.style.display = nestedTable.style.display === 'none' || nestedTable.style.display === '' ? 'table-row' : 'none';
  }

    function selectRow(row) {
        const selectedData = row.cells;
        const nestedTable = row.closest('table');
        const parentRowId = nestedTable.closest('tr').getAttribute('data-parent');
        if (!parentRowId) {
            alert("Ошибка");
            return;
        }
        const mainRow = document.getElementById(parentRowId);
        if (!mainRow){
        alert("Ошибка");
            return;
        }

        // Bits
        mainRow.cells[1].textContent = selectedData[0].textContent; // C1Y
        mainRow.cells[2].textContent = selectedData[1].textContent; // C2Y
        mainRow.cells[3].textContent = selectedData[2].textContent; // C3Y
        // Permissions
        mainRow.cells[4].textContent = selectedData[3].textContent; // Read
        mainRow.cells[5].textContent = selectedData[4].textContent; // Write
        mainRow.cells[6].textContent = selectedData[5].textContent; // Increment
        mainRow.cells[7].textContent = selectedData[6].textContent; // Decrement/Transfer/Restore
        mainRow.cells[8].textContent = selectedData[7].textContent; // Remarks

        encode();
        row.classList.add('selected-row');
        setTimeout(() => row.classList.remove('selected-row'), 1000);
    }

    function selectStRow(row) {
        const selectedData = row.cells;
        const nestedTable = row.closest('table');
        const parentRowId = nestedTable.closest('tr').getAttribute('data-parent');

        if (!parentRowId) {
            alert('Ошибка');
            return;
        }

        const mainRow = document.getElementById(parentRowId);
        if (!mainRow) {
            alert('Ошибка');
            return;
        }

        // Bits:
        mainRow.cells[0].textContent = selectedData[0].textContent;
        mainRow.cells[1].textContent = selectedData[1].textContent;
        mainRow.cells[2].textContent = selectedData[2].textContent; 
        // permissions
        mainRow.cells[3].textContent = selectedData[3].textContent; 
        mainRow.cells[4].textContent = selectedData[4].textContent;
        mainRow.cells[5].textContent = selectedData[5].textContent; 
        mainRow.cells[6].textContent = selectedData[6].textContent;
        mainRow.cells[7].textContent = selectedData[7].textContent;
        mainRow.cells[8].textContent = selectedData[8].textContent;
        mainRow.cells[9].textContent = selectedData[9].textContent;

        encode();
        row.classList.add('selected-row');
        setTimeout(() => row.classList.remove('selected-row'), 1000);
  }

    document.getElementById('access-bits').addEventListener('input', decode);

    const DATA_ACCESS_BYTES = {
        "0 0 0": {
            read: "ключ A|B",
            write: "ключ A|B",
            increment: "ключ A|B",
            decrementTransferRestore: "ключ A|B",
            remarks: "Конфигурация по умолчанию"
        },
        "0 1 0": {
            read: "ключ A|B",
            write: "никогда",
            increment: "никогда",
            decrementTransferRestore: "никогда",
            remarks: "блок чтения/записи"
        },
        "1 0 0": {
            read: "ключ A|B",
            write: "ключ B",
            increment: "никогда",
            decrementTransferRestore: "никогда",
            remarks: "блок чтения/записи"
        },
        "1 1 0": {
            read: "ключ A|B",
            write: "ключ B",
            increment: "ключ B",
            decrementTransferRestore: "ключ A|B",
            remarks: "блок значения"
        },
        "0 0 1": {
            read: "ключ A|B",
            write: "никогда",
            increment: "никогда",
            decrementTransferRestore: "ключ A|B",
            remarks: "блок значения"
        },
        "0 1 1": {
            read: "ключ B",
            write: "ключ B",
            increment: "никогда",
            decrementTransferRestore: "никогда",
            remarks: "блок чтения/записи"
        },
        "1 0 1": {
            read: "ключ B",
            write: "никогда",
            increment: "никогда",
            decrementTransferRestore: "никогда",
            remarks: "блок чтения/записи"
        },
        "1 1 1": {
            read: "никогда",
            write: "никогда",
            increment: "никогда",
            decrementTransferRestore: "никогда",
            remarks: "блок чтения/записи"
        }
    };

    const TRAILER_ACCESS_BYTES = {
        "0 0 0": {
            readKeyA: "никогда",
            writeKeyA: "ключ A",
            readAccessBits: "ключ A",
            writeAccessBits: "никогда",
            readKeyB: "ключ A",
            writeKeyB: "ключ A",
            remarks: "Ключ B можно прочитать"
        },
        "0 1 0": {
            readKeyA: "никогда",
            writeKeyA: "никогда",
            readAccessBits: "ключ A",
            writeAccessBits: "никогда",
            readKeyB: "ключ A",
            writeKeyB: "никогда",
            remarks: "Ключ B можно прочитать"
        },
        "1 0 0": {
            readKeyA: "никогда",
            writeKeyA: "ключ B",
            readAccessBits: "ключ A|B",
            writeAccessBits: "никогда",
            readKeyB: "никогда",
            writeKeyB: "ключ B",
            remarks: ""
        },
        "1 1 0": {
            readKeyA: "никогда",
            writeKeyA: "никогда",
            readAccessBits: "ключ A|B",
            writeAccessBits: "никогда",
            readKeyB: "никогда",
            writeKeyB: "никогда",
            remarks: ""
        },
        "0 0 1": {
            readKeyA: "никогда",
            writeKeyA: "ключ A",
            readAccessBits: "ключ A",
            writeAccessBits: "ключ A",
            readKeyB: "ключ A",
            writeKeyB: "ключ A",
            remarks: "Ключ B можно прочитать; Конфигурация по умолчанию"
        },
        "0 1 1": {
            readKeyA: "никогда",
            writeKeyA: "ключ B",
            readAccessBits: "ключ A|B",
            writeAccessBits: "ключ B",
            readKeyB: "никогда",
            writeKeyB: "ключ B",
            remarks: ""
        },
        "1 0 1": {
            readKeyA: "никогда",
            writeKeyA: "никогда",
            readAccessBits: "ключ A|B",
            writeAccessBits: "ключ B",
            readKeyB: "никогда",
            writeKeyB: "никогда",
            remarks: ""
        },
        "1 1 1": {
            readKeyA: "никогда",
            writeKeyA: "никогда",
            readAccessBits: "ключ A|B",
            writeAccessBits: "никогда",
            readKeyB: "никогда",
            writeKeyB: "никогда",
            remarks: ""
        }
    };

    function showError(message) {
        const errorBox = document.getElementById('error-box');
        errorBox.textContent = message; 
        errorBox.style.display = 'inline'; 
    }

    function hideError() {
        const errorBox = document.getElementById('error-box');
        errorBox.style.display = 'none';
    }

    // Function to determine permissions and acess from the bytes
    function decode() {
        const accessBits = document.getElementById('access-bits').value;

        if (accessBits.length < 6) {
            return;
        } else if (accessBits.length > 6) {
            showError("Некорректные биты доступа");
        }

        // console.log("Decoded Access Bits: ", accessBits);

        const byte6 = accessBits.slice(0, 2); // Byte 6
        const byte7 = accessBits.slice(2, 4); // Byte 7
        const byte8 = accessBits.slice(4, 6); // Byte 8

        // Convert the byte strings to integers (assuming hexadecimal)
        const byteArray = [
            parseInt(byte6, 16), // Convert byte6 to an integer
            parseInt(byte7, 16), // Convert byte7 to an integer
            parseInt(byte8, 16)  // Convert byte8 to an integer
        ];

        // Function to convert a byte to its corresponding bits array
        const byteToBits = (byte) => {
            return Array.from({ length: 8 }, (_, i) => (byte >> (7 - i)) & 1);
        };

        // Create a dictionary with byte keys and their corresponding bits array
        const byteToBitsDict = {
            "byte6": byteToBits(byteArray[0]),
            "byte7": byteToBits(byteArray[1]),
            "byte8": byteToBits(byteArray[2])
        };
        
        let Cxy = {
            "C10": byteToBitsDict.byte7[3],
            "C20": byteToBitsDict.byte8[7],
            "C30": byteToBitsDict.byte8[3],

            "C11": byteToBitsDict.byte7[2],
            "C21": byteToBitsDict.byte8[6],
            "C31": byteToBitsDict.byte8[2],

            "C12": byteToBitsDict.byte7[1],
            "C22": byteToBitsDict.byte8[5],
            "C32": byteToBitsDict.byte8[1],

            "C13": byteToBitsDict.byte7[0],
            "C23": byteToBitsDict.byte8[4],
            "C33": byteToBitsDict.byte8[0],
        };

        let ICxy = {
            // Inverted values
            "IC10": byteToBitsDict.byte6[7],
            "IC20": byteToBitsDict.byte6[3],
            "IC30": byteToBitsDict.byte7[7],

            "IC11": byteToBitsDict.byte6[6],
            "IC21": byteToBitsDict.byte6[2],
            "IC31": byteToBitsDict.byte7[6],

            "IC12": byteToBitsDict.byte6[5],
            "IC22": byteToBitsDict.byte6[1],
            "IC32": byteToBitsDict.byte7[5],

            "IC13": byteToBitsDict.byte6[4],
            "IC23": byteToBitsDict.byte6[0],
            "IC33": byteToBitsDict.byte7[4],
        };
        
        for (let key in Cxy) {
            let invertedKey = "I" + key; 
            let invertedBit = Cxy[key] ^ 1;
            if (invertedBit !== ICxy[invertedKey]) {
                // console.log(key, invertedKey, Cxy[key], ICxy[invertedKey]);
                showError("Некорректные биты доступа");
                return ;
            }
        }

        hideError();

        // console.log(byteToBitsDict);
        // let blockAccessBits = {
        //     "block0": byteToBitsDict.byte7[3] + " " + byteToBitsDict.byte8[7] + " " + byteToBitsDict.byte8[3],
        //     "block1": byteToBitsDict.byte7[2] + " " + byteToBitsDict.byte8[6] + " "+ byteToBitsDict.byte8[2],
        //     "block2": byteToBitsDict.byte7[1] + " " + byteToBitsDict.byte8[5] + " " + byteToBitsDict.byte8[1],
        //     "block3": byteToBitsDict.byte7[0] + " " + byteToBitsDict.byte8[4] + " " + byteToBitsDict.byte8[0],
        // }
        let blockAccessBits = {
            "block0": Cxy.C10 + " " + Cxy.C20 + " " + Cxy.C30,
            "block1": Cxy.C11 + " " + Cxy.C21 + " " + Cxy.C31,
            "block2": Cxy.C12 + " " + Cxy.C22 + " " + Cxy.C32,
            "block3": Cxy.C13 + " " + Cxy.C23 + " " + Cxy.C33,
        }

        for (let i = 0; i <= 2; i++) {
            let blockCols = DATA_ACCESS_BYTES[blockAccessBits[`block${i}`]];
            let row = document.getElementById(`db-row${i}`);

            row.cells[1].textContent = Cxy[`C1${i}`];
            row.cells[2].textContent = Cxy[`C2${i}`];
            row.cells[3].textContent = Cxy[`C3${i}`];
            row.cells[4].textContent = blockCols.read;
            row.cells[5].textContent = blockCols.write;
            row.cells[6].textContent = blockCols.increment;
            row.cells[7].textContent = blockCols.decrementTransferRestore;
            row.cells[8].textContent = blockCols.remarks;
        }

        let stCols = TRAILER_ACCESS_BYTES[blockAccessBits.block3];
        const stRow = document.getElementById('st-row0');
        stRow.cells[0].textContent = Cxy.C13;
        stRow.cells[1].textContent = Cxy.C23;
        stRow.cells[2].textContent = Cxy.C33;
        stRow.cells[3].textContent = stCols.readKeyA;
        stRow.cells[4].textContent = stCols.writeKeyA;
        stRow.cells[5].textContent = stCols.readAccessBits;
        stRow.cells[6].textContent = stCols.writeAccessBits;
        stRow.cells[7].textContent = stCols.readKeyB;
        stRow.cells[8].textContent = stCols.writeKeyB;
        stRow.cells[9].textContent = stCols.remarks;
    }

    // convert access and permission to bytes
    function encode() {
        let byte6 = 0xFF;
        let byte7 = 0x07;
        let byte8 = 0x80;

        let block0 = document.getElementById("db-row0");
        let block1 = document.getElementById("db-row1");
        let block2 = document.getElementById("db-row2");
        let block3 = document.getElementById("st-row0");

        let Cxy = {
            "C10": block0.cells[1].textContent,
            "C20": block0.cells[2].textContent,
            "C30": block0.cells[3].textContent,

            "C11": block1.cells[1].textContent,
            "C21": block1.cells[2].textContent,
            "C31": block1.cells[3].textContent,

            "C12": block2.cells[1].textContent,
            "C22": block2.cells[2].textContent,
            "C32": block2.cells[3].textContent,

            "C13": block3.cells[0].textContent,
            "C23": block3.cells[1].textContent,
            "C33": block3.cells[2].textContent,
        };

        let ICxy = {
            // Inverted values
            "IC10": Cxy.C10 ^ 1,
            "IC20": Cxy.C20 ^ 1,
            "IC30": Cxy.C30 ^ 1,

            "IC11": Cxy.C11 ^ 1,
            "IC21": Cxy.C21 ^ 1,
            "IC31": Cxy.C31 ^ 1,

            "IC12": Cxy.C12 ^ 1,
            "IC22": Cxy.C22 ^ 1,
            "IC32": Cxy.C32 ^ 1,

            "IC13": Cxy.C13 ^ 1,
            "IC23": Cxy.C23 ^ 1,
            "IC33": Cxy.C33 ^ 1,
        };

        byte6 = 
            ((ICxy.IC23 & 0x1) << 7) | 
            ((ICxy.IC22 & 0x1) << 6) | 
            ((ICxy.IC21 & 0x1) << 5) | 
            ((ICxy.IC20 & 0x1) << 4) |
            ((ICxy.IC13 & 0x1) << 3) | 
            ((ICxy.IC12 & 0x1) << 2) | 
            ((ICxy.IC11 & 0x1) << 1) | 
            ((ICxy.IC10 & 0x1) << 0);

        byte7 = 
            ((Cxy.C13 & 0x1) << 7) | 
            ((Cxy.C12 & 0x1) << 6) | 
            ((Cxy.C11 & 0x1) << 5) | 
            ((Cxy.C10 & 0x1) << 4) |
            ((ICxy.IC33 & 0x1) << 3) | 
            ((ICxy.IC32 & 0x1) << 2) | 
            ((ICxy.IC31 & 0x1) << 1) | 
            ((ICxy.IC30 & 0x1) << 0);

        byte8 = 
            ((Cxy.C33 & 0x1) << 7) | 
            ((Cxy.C32 & 0x1) << 6) | 
            ((Cxy.C31 & 0x1) << 5) | 
            ((Cxy.C30 & 0x1) << 4) |
            ((Cxy.C23 & 0x1) << 3) | 
            ((Cxy.C22 & 0x1) << 2) | 
            ((Cxy.C21 & 0x1) << 1) | 
            ((Cxy.C20 & 0x1) << 0);

        let finalAccessBits = byte6.toString(16).toUpperCase().padStart(2, '0') +
            byte7.toString(16).toUpperCase().padStart(2, '0') +
            byte8.toString(16).toUpperCase().padStart(2, '0');

        document.getElementById('access-bits').value = finalAccessBits;
    }
</script>

## Ссылки
- Интерфейс вдохновлён этим калькулятором: [Mifarecalc](https://gitlab.com/limentas/mifare-calc)
- [MIFARE-Classic-1K-Access-Bits-Calculator ](https://github.com/akafugu/MIFARE-Classic-1K-Access-Bits-Calculator)
