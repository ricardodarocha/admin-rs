
interface codigoibgeType {
}

interface estadoType {
   codigo: number;
   nome: string;
   siglauf: string;
}

interface rootElementItemType {
   codigo: number;
   codigoestado: number;
   uf: string;
   nome: string;
   codigoibge: codigoibgeType;
   estado: estadoType;
}
